use defs::{ Action, Event, DieEvent };
use strings::{ CANNON, RADAR, MOVE, HIT, SEE, RADARECHO, DIE };
use position::Pos;
use patterns::*;
use ai::*;
use ai::bot::Bot;
use lists::*;
use lists::ActionMode::*;
use std::cmp::max;

impl Ai {
    // Will alternate between all bots shooting at the last echo and 1 bot scanning
    // while the rest free bots shoots
    // WFT are exit codes?
    // Well, I noticed the strategy consisted of several steps where we do some stuff and then
    // maybe return if a condition is met. So, I extracted those steps and now they are
    // functions that will return Some(exit_code) if they wish to end there with that code,
    // or None if they wish to let the next step run. 
    pub fn aggressive_attack_strategy(&mut self, mut actions: &mut Vec<Action>) -> bool {
        if let Some(exc) = self.killed_exit_code()               { return exc; }
        if let Some(exc) = self.shoot_echoes_exit_code(actions)  { return exc; }
        if let Some(exc) = self.shoot_hits_exit_code(actions)    { return exc; }
        if let Some(exc) = self.shoot_history_exit_code(actions) { return exc; }
        return false;
    }

    // Don't continue to attack if we killed something
    // TODO: Look at hit events with this bot_id, then get the pos of that hit id
    // and check if we have other radar echoes to pursue
    fn killed_exit_code(&mut self) -> Option<bool> {
        if let Some(ev) = self.get_possible_kill() {
            println!("We killed something, back to scanning: {:?}", ev);
            self.logger.log(&format!("We killed something, back to scanning: {:?}", ev), 2);
            return Some(false);
        } else {
            return None;
        }
    }

    fn shoot_echoes_exit_code(&mut self, mut actions: &mut Vec<Action>) -> Option<bool> {
        // Are there any echoes this round? shoot at them...
        let see_positions_this_round = self.history.get_echo_positions(1);
        println!("See positions this round {:?}", see_positions_this_round);
        if see_positions_this_round.len() > 0 {
            //TODO: Handle multiple seen ones
            println!("Radar position found this round {:?}", see_positions_this_round[0].0);

            //TODO: Don't do this if we've already found all asteroids
            // Because of asteroids we want to make sure that the first time we see something
            // We scan as we shoot so we can mark detect asteroids
            if self.last_action_mode() == Scan {
                self.attack_and_scan_pos(&mut actions, see_positions_this_round[0].0);
            } else {
                self.attack_pos(&mut actions, see_positions_this_round[0].0);
            }

            self.log_attack_actions(&actions, "have fresh seen data");
            return Some(true);
        }
        return None;
    }

    fn shoot_hits_exit_code(&mut self, mut actions: &mut Vec<Action>) -> Option<bool> {
        // Gets tuples of (Event,round_id) from hit events in the last n rounds
        let hit_events_this_round = self.hits_on_enemies(1);
        if hit_events_this_round.len() > 0 {
            if let Some(pos) = self.get_pos_from_hit(&hit_events_this_round[0].0, self.round_id) {
                println!("Found pos of last hit, attacking {:?}", pos);
                self.attack_pos(&mut actions, pos);
                self.log_attack_actions(&actions, "have fresh hit data");
                return Some(true);
            }
        }
        return None;
    }

    fn shoot_history_exit_code(&mut self, mut actions: &mut Vec<Action>) -> Option<bool> {
        // So far we have not really used the mode field because we've had fresh data
        // of something to shoot at, this is where we look if we are in attack mode
        // but just had some bad luck last round
        if self.last_action_mode() == Attack {
            println!("We were attacking last round, let's continue with that if we can");

            let exc_opt = self.see_lastround_exit_code(actions);
            if exc_opt.is_some() {return exc_opt;}

            let exc_opt = self.hit_lastround_exit_code(actions);
            if exc_opt.is_some() {return exc_opt;}
        }
        return None;
    }


    fn hit_lastround_exit_code(&mut self, mut actions: &mut Vec<Action>) -> Option<bool> {
        let hit_events = self.hits_on_enemies(5);
        let hit_events_last_round = hit_events.iter().cloned()
            .filter(|tup|tup.1 == self.last_round()).collect::<Vec<(Event,i16)>>();
        if hit_events_last_round.len() > 0 {
            println!("Found hit event last round {:?}", hit_events_last_round[0]);
            if let Some(pos) = self.get_pos_from_hit(&hit_events_last_round[0].0, self.last_round()) {
                println!("Pos of last round hit {:?}", pos);
                self.attack_and_scan_pos(&mut actions, pos);
                self.log_attack_actions(&actions, "have one round old hit data");
                return Some(true);
            }
        }
        return None;
    }

    fn see_lastround_exit_code(&mut self, mut actions: &mut Vec<Action>) -> Option<bool> {
        // Since we got here we know we have no echoes or hits this round,
        // how about last round?
        let see_positions = self.history.get_echo_positions(2);
        let see_positions_last_round = see_positions.iter()
            .filter(|tup|!self.asteroids.is_asteroid(tup.0))
            .filter(|tup|tup.1 == self.last_round()).collect::<Vec<_>>();

        if see_positions_last_round.len() > 0 {
            println!("Radar position found last round {:?}", see_positions_last_round[0].0);
            self.attack_and_scan_pos(&mut actions, see_positions_last_round[0].0);
            self.log_attack_actions(&actions, "have one round old seen data");
            return Some(true);
        }
        return None;
    }

    fn hits_on_enemies(&self, since: i16) -> Vec<(Event, i16)> {
        self.history.get_events( HIT, since )
            .iter()
            .cloned()
            .filter(|tup|{
                match tup.0 {
                    Event::Hit(ev) => {
                        !self.is_our_bot(ev.bot_id)
                    }
                    _ => false,
                }
            })
            .collect()
    }

    fn last_action_mode(&self) -> ActionMode {
        let round_entry = self.history.get(&self.last_round());
        match round_entry {
            Some(entry) => {
                return entry.mode;
            },
            None => {
                return Nomode;
            }
        }
    }

    fn last_round(&self) -> i16 {
        //if self.round_id - 1 >= 0 { self.round_id - 1 } else { 0 };
        max(self.round_id - 1, 0)
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
        //let available_bots = self.get_live_bots();
        let available_bots = self.draft_healthy();
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

    fn draft_healthy(&self) -> Vec<Bot> {
        self.get_live_bots().into_iter()
            .filter(|bot| bot.is_healthy())
            .collect()
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

    // Returns one die event on enemy bot from this round is there was one. Else None.
    #[allow(dead_code)]
    fn get_possible_kill(&self) -> Option<DieEvent> {
        for entry in self.history.get_events( DIE, 1 ) {
            match entry.0 {
                Event::Die(ref ev) => {
                    if !self.is_our_bot(ev.bot_id) {
                        return Some(ev.clone());
                    }
                },
                _ => ()
            }
        }
        return None;
    }
}
