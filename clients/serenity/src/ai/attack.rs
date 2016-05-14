use defs::{ Action, Event, DieEvent };
use strings::{ CANNON, RADAR, MOVE, HIT, SEE, RADARECHO, DIE, MODE_ATTACK, MODE_SCAN, NOACTION };
use position::Pos;
use ai::*;
use lists::*;
use ai::bot::Bot;

impl Ai {
    // Will alternate between all bots shooting at the last echo and 1 bot scanning
    // while the rest free bots shoots
    // Returns a Some(attack_mode) or None TODO: Maybe attack mode should be an enum?
    pub fn aggressive_attack_strategy(&mut self, mut actions: &mut Vec<Action>) -> bool {
        let last_mode;
        let last_round = if self.round_id - 1 >= 0 { self.round_id - 1 } else { 0 };
        {
            let round_entry = self.history.get(&last_round);
            match round_entry {
                Some(entry) => {
                    last_mode = entry.mode.clone();
                },
                None => {
                    last_mode = NOACTION.to_string();
                }
            }
        }
        println!("Last mode {:?}", last_mode);

        // Gets tuples of (Pos,round_id) from echo/see events in the last n rounds
        let see_positions = self.history.get_echo_positions(5);

        // Don't continue to attack if we killed something
        // TODO: Look hit events with this bot_id, then get the pos of that hit id
        // and check if we have other radar echoes to pursue
        if let Some(ev) = self.get_possible_die_event() {
            println!("We killed something, back to scanning: {:?}", ev);
            return false;
        }

        // Are there any echoes this round? shoot at them...
        let see_positions_this_round = see_positions.iter()
            .filter(|tup|tup.1 == self.round_id).collect::<Vec<_>>();
        if see_positions_this_round.len() > 0 {
            println!("Radar position found this round {:?}", see_positions_this_round[0].0.clone());
            self.attack_pos(&mut actions, see_positions_this_round[0].0.clone());
            return true;
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
            if let Some(pos) = self.get_pos_from_hit(&hit_events_this_round[0].0, &self.round_id) {
                println!("Found pos of last hit, attacking {:?}", pos);
                self.attack_pos(&mut actions, pos.clone());
                return true;
            }
        }

        // So far we have not really used the mode field because we've had fresh data
        // of something to shoot at, this is where we look if we are in attack mode
        // but just had some bad luck last round
        if last_mode == MODE_ATTACK.to_string() {
            println!("We were attacking last round, let's continue with that");
            // Since we got here we know we have no echoes or hits this round,
            // how about last round?
            let see_positions_last_round = see_positions.iter()
                .filter(|tup|tup.1 == last_round).collect::<Vec<_>>();
            if see_positions_last_round.len() > 0 {
                println!("Radar position found last round {:?}", see_positions_last_round[0].0.clone());
                self.attack_and_scan_pos(&mut actions, see_positions_last_round[0].0.clone());
                return true;
            }

            // How about hit events last round?
            let hit_events_last_round = hit_events.iter().cloned()
                .filter(|tup|tup.1 == last_round).collect::<Vec<(Event,i16)>>();
            if hit_events_last_round.len() > 0 {
                println!("Found hit event last round {:?}", hit_events_last_round[0]);
                if let Some(pos) = self.get_pos_from_hit(&hit_events_last_round[0].0, &last_round) {
                    println!("Pos of last round hit {:?}", pos);
                    self.attack_and_scan_pos(&mut actions, pos.clone());
                    return true;
                }
            }

        }

        return false;
    }

    // Will just attack a position with all we've got
    // Typical use: We have a fresh echo or hit (from this round)
    pub fn attack_pos(&mut self, mut actions: &mut Vec<Action>, target: Pos) {
        let radius = self.config.field_radius;
        let bots_alive = self.bots_alive() as i16;
        self.bots.iter_mut()
            .zip(target.smart_attack_spread(bots_alive))
            .map(|(&mut ref bot, ref pos)| actions.set_action_for(bot.id, CANNON, pos.clamp(&radius)))
            .count();
    }

    // Will attack a position, but also make sure we scan it to not lose track of the target
    // Typical use: We have a 1 round old echo or hit
    pub fn attack_and_scan_pos(&mut self, mut actions: &mut Vec<Action>, target: Pos) {
        // Let's first get available bots, I'm going to filter out bots on the move
        // but this is optional depending on how aggressive we want to be
        let available_bots = self.get_live_bots()
            .iter()
            .cloned()
            .filter(|bot| {
                if Some(actions.get_action(bot.id)).unwrap().unwrap().action_type != MOVE.to_string() {
                    true
                } else {
                    false
                }
            })
            .collect::<Vec<_>>();

        // Let's generate a positions vector to zip into our available_bots vector
        // The first position will be the target position for radar purposes,
        // and the following will be attack positions
        let available_bot_count = available_bots.len();
        let mut positions: Vec<Pos> = vec![target];
        positions.append(&mut target.smart_attack_spread(available_bot_count as i16));
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
                    Event::See(ref ev) => (ev.pos.clone(), tup.1),
                    Event::Echo(ref ev) => (ev.pos.clone(), tup.1),
                    _ => (Pos::default(), 0)
                }
            })
            .filter(|&(ref pos, ref round_id)| pos.distance(target) <= max_radius)
            .fold(None, |acc, curr| {
                if let Some(a) = acc {
                    if curr.1 >= a.1 { Some(curr) } else { Some(a) }
                } else {
                    Some(curr)
                }
            })
    }

    // Returns the number of bots that are alive and not evading
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

    // See if we need this for anything, with the current logic probably not
    // Maybe some edge cases with the one bot strategy?
    fn get_pos_from_hit(&self, hit_event: &Event, round_id: &i16) -> Option<Pos> {
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
