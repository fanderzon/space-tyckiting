use defs::{ Action, Event, DieEvent };
use strings::{ CANNON, RADAR, MOVE, HIT, SEE, RADARECHO, DIE };
use position::Pos;
use patterns::*;
use ai::*;
use lists::*;
use lists::ActionMode::*;

impl Ai {
    // Will alternate between all bots shooting at the last echo and 1 bot scanning
    // while the rest free bots shoots
    // Returns a Some(attack_mode) or None TODO: Maybe attack mode should be an enum?
    pub fn aggressive_attack_strategy(&mut self, mut actions: &mut Vec<Action>, decision: &mut Decision) {
        let last_mode;
        let last_round = if self.round_id - 1 >= 0 { self.round_id - 1 } else { 0 };
        {
            let round_entry = self.history.get(&last_round);
            match round_entry {
                Some(entry) => {
                    last_mode = entry.decision.mode;
                },
                None => {
                    last_mode = Nomode;
                }
            }
        }
        println!("Last mode {:?}", last_mode);

        // Gets tuples of (Pos,round_id) from echo/see events in the last n rounds
        let see_positions = self.history.get_echo_positions(5);

        // Don't continue to attack if we killed something
        // TODO: Look at hit events with this bot_id, then get the pos of that hit id
        // and check if we have other radar echoes to pursue
        if let Some(ev) = self.get_possible_die_event() {
            println!("We killed something, back to scanning: {:?}", ev);
            return ();
        }

        // Are there any echoes this round? shoot at them...
        let see_positions_this_round = see_positions
            .iter()
            .filter_map(|tup| {
                if tup.1 == self.round_id {
                    Some(tup.0)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        println!("See positions this round {:?}", see_positions_this_round);
        //TODO: Handle multiple seen ones
        if see_positions_this_round.len() > 0 {
            println!("Radar position found this round {:?}", see_positions_this_round[0]);

            //TODO: Don't do this if we've already found all asteroids
            // Because of asteroids we want to make sure that the first time we see something
            // We scan as we shoot so we can mark detect asteroids
            if last_mode == Scan {
                decision.add_attack_decision(&see_positions_this_round[0], &see_positions_this_round);
                self.attack_and_scan_pos(&mut actions, see_positions_this_round[0]);
            } else {
                decision.add_attack_decision(&see_positions_this_round[0], &see_positions_this_round);
                self.attack_pos(&mut actions, see_positions_this_round[0]);
            }

            self.log_attack_actions(&actions, "have fresh seen data");
            return ();
        }

        // Gets tuples of (Event,round_id) from hit events in the last n rounds
        let hit_events = self.history.get_events( HIT, 5 )
            .iter()
            .cloned()
            .filter(|tup|{
                match tup.0 {
                    Event::Hit(ev) => {
                        !self.is_our_bot(ev.bot_id)
                    }
                    _ => false,
                }
            }).collect::<Vec<(Event,i16)>>();


        // Are there any hit events this round, continue shooting
        let hit_events_this_round = hit_events.iter().cloned()
            .filter(|tup| tup.1 == self.round_id).collect::<Vec<(Event,i16)>>();
        if hit_events_this_round.len() > 0 {
            if let Some(pos) = self.get_pos_from_hit(&hit_events_this_round[0].0, self.round_id) {
                println!("Found pos of last hit, attacking {:?}", pos);
                decision.add_attack_decision(&pos, &vec![]);
                self.attack_pos(&mut actions, pos);
                self.log_attack_actions(&actions, "have fresh hit data");
                return ();
            }
        }

        // So far we have not really used the mode field because we've had fresh data
        // of something to shoot at, this is where we look if we are in attack mode
        // but just had some bad luck last round
        if last_mode == Attack {
            println!("We were attacking last round, let's continue with that if we can");
            // Since we got here we know we have no echoes or hits this round,
            // how about last round?
            let see_positions_last_round = see_positions
                .iter()
                .filter_map(|tup| {
                    if !self.asteroids.is_asteroid(tup.0) && tup.1 == last_round {
                        Some(tup.0)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if see_positions_last_round.len() > 0 {
                println!("Radar position found last round {:?}", see_positions_last_round[0]);
                decision.add_attack_decision(&see_positions_last_round[0], &see_positions_last_round);
                self.attack_and_scan_pos(&mut actions, see_positions_last_round[0]);
                self.log_attack_actions(&actions, "have one round old seen data");
                return ();
            }

            // How about hit events last round?
            let hit_events_last_round = hit_events.iter().cloned()
                .filter(|tup|tup.1 == last_round).collect::<Vec<(Event,i16)>>();
            if hit_events_last_round.len() > 0 {
                println!("Found hit event last round {:?}", hit_events_last_round[0]);
                if let Some(pos) = self.get_pos_from_hit(&hit_events_last_round[0].0, last_round) {
                    println!("Pos of last round hit {:?}", pos);
                    decision.add_attack_decision(&pos, &vec![]);
                    self.attack_and_scan_pos(&mut actions, pos);
                    self.log_attack_actions(&actions, "have one round old hit data");
                    return ();
                }
            }
        }

        return ();
    }

    fn log_attack_actions(&mut self, actions: &Vec<Action>, motivation: &str, ) {
        for action in actions {
            self.logger.log(&format!("Attack with Bot {} on {} b/c {}.", action.bot_id, action.pos, motivation), 2);
        }
    }

    // Will just attack a position with all we've got
    // Typical use: We have a fresh echo or hit (from this round)
    #[allow(dead_code)]
    pub fn attack_pos(&mut self, actions: &mut Vec<Action>, target: Pos) {
        let radius = self.config.field_radius;
        let available_bots = self.get_live_bots();
        let bots_alive = available_bots.len() as i16;

        available_bots.iter()
            .zip(smart_attack_spread(target, bots_alive))
            .map(|(&ref bot, ref pos)| actions.set_action_for(bot.id, CANNON, pos.clamp(&radius)))
            .count();
    }

    // Will attack a position, but also make sure we scan it to not lose track of the target
    // Typical use: We have a 1 round old echo or hit
    pub fn attack_and_scan_pos(&mut self, actions: &mut Vec<Action>, target: Pos) {
        println!("attack_and_scan_pos: avilable bots {:?}", self.get_live_bots());
        // Let's first get available bots, I'm going to filter out bots on the move
        // but this is optional depending on how aggressive we want to be
        let available_bots = self.get_live_bots()
            .iter()
            .cloned()
            .filter(|bot| {
                actions.get_action(bot.id).unwrap().action_type != MOVE.to_string()
            })
            .collect::<Vec<_>>();

        // Let's generate a positions vector to zip into our available_bots vector
        // The first position will be the target position for radar purposes,
        // and the following will be attack positions
        let available_bot_count = available_bots.len();
        let mut positions: Vec<Pos> = vec![target];
        positions.append(&mut smart_attack_spread(target, available_bot_count as i16));
        let mut radared = false;

        println!("Attacking or scanning with {:?} bots, to: {:?}", available_bot_count, positions);

        available_bots
            .iter()
            .zip(positions)
            .map(|(&ref bot, ref pos)| {
                if !radared {
                    actions.set_action_for(bot.id, RADAR, *pos);
                    radared = true;
                } else {
                    actions.set_action_for(bot.id, CANNON, *pos);
                }
            })
            .count();
    }

    // Give a position, get back the latest echo/see position within your max_radius
    // and what round it was
    // Might be useful?
    #[allow(dead_code)]
    fn find_echo_within_radius(&self, target: Pos, max_radius: i16) -> Option<(Pos,i16)> {
        // get relevant events 10 rounds back
        let mut see_events = self.history.get_events( SEE, 10 );
        see_events.append(&mut self.history.get_events( RADARECHO, 10 ));

        see_events
            .iter()
            .cloned()
            .map(|tup| {
                match tup.0 {
                    Event::See(ref ev) => (ev.pos, tup.1),
                    Event::Echo(ref ev) => (ev.pos, tup.1),
                    _ => (Pos::origo(), 0)
                }
            })
            .filter(|&(ref pos, _)| pos.distance(target) <= max_radius)
            .fold(None, |acc, curr| {
                if let Some(a) = acc {
                    if curr.1 >= a.1 { Some(curr) } else { Some(a) }
                } else {
                    Some(curr)
                }
            })
    }

    // Returns the number of bots that are alive and not evading
    #[allow(dead_code)]
    pub fn bots_available_for_attack(&self, actions: &Vec<Action>) -> usize {
        self.bots
            .iter()
            .filter(|bot| bot.alive && {
                if let Some(ac) = actions.iter().find(|ac| ac.bot_id == bot.id) {
                    ac.action_type != MOVE.to_string()
                } else {
                    false
                }
            })
            .count()
    }

    // Hit events don't have positions, but they have the source (bot_id) of the bot
    // that made the shot, so you can go back one round and get that bots cannon action
    // That means that the position this method returns will not have a 1:1 relationship
    // with the enemy bot position, but can be anywhere within the cannon radius (1)
    pub fn get_pos_from_hit(&self, hit_event: &Event, round_id: i16) -> Option<Pos> {
        let previous_round: i16 = round_id - 1;
        let mut source: i16 = -1;
        match hit_event {
            &Event::Hit(ref ev) => source = ev.source,
            _ => ()
        };
        println!("get_pos_from_hit_entry previous round {:?} source {}", previous_round, source);
        let cannons = self.history
            .get_actions_for_round( CANNON, previous_round )
            .iter()
            .cloned()
            .filter(|ac| ac.bot_id == source)
            .collect::<Vec<Action>>();
            println!("Cannon matches {:?}", cannons);
            println!("Cannon actions {:?}", self.history.get_actions_for_round( CANNON, previous_round ));
        // Should be guaranteed to have 1 cannon match
        if cannons.len() > 0 {
            return Some(cannons[0].pos);
        } else {
            return None;
        }
    }

    // Convenience for acting on an enemy die event, needed?
    #[allow(dead_code)]
    fn get_possible_die_event(&self) -> Option<DieEvent> {
        let die_events = self.history.get_events( DIE, 1 );
        if die_events.len() > 0 {
            for entry in die_events {
                let dead = entry.0;
                match dead {
                    Event::Die(ref ev) => {
                        if !self.is_our_bot(ev.bot_id) {
                            return Some(ev.clone());
                        }
                    },
                    _ => ()
                }
            }
        }
        return None;
    }
}
