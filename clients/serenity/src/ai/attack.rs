use defs::{ Action, Event, DieEvent };
use strings::{ CANNON, RADAR, MOVE, SEE, RADARECHO, DIE, MODE_ATTACK };
use position::Pos;
use ai::*;
use lists::*;

impl Ai {
    pub fn attack_and_scan_if_target(&self, mut actions: &mut Vec<Action>) -> bool {
        let mut target: Option<Pos> = None;
        // println!("Bots available for attack {:?}", self.bots_available_for_attack(&actions));
        // Let's see when the last time was we knew about an enemy position
        // println!("Last enemy position {:?}", self.history.get_last_enemy_position());

        // Let's look for the easiest case first, if we saw an enemy last turn
        let mut attack_events = self.history.get_events_for_round( SEE, self.round_id );
        attack_events.append(&mut self.history.get_events_for_round( RADARECHO, self.round_id ));
        for t in attack_events {
            match t {
                Event::See(ref ev) => target = Some(ev.pos),
                Event::Echo(ref ev) => target = Some(ev.pos),
                _ => ()
            }
        }

        // Separate the logic needed if we only have one bot left
        if self.bots_alive() == 1 {
            return self.one_bot_attack_strategy(&mut actions, target);
        } else {
            if let Some(t) = target {
                if self.in_attack_scan_loop() {
                    println!("In scan loop");
                    self.one_bot_attack_strategy(&mut actions, target);
                } else {
                    self.all_shoot_or_scan(&mut actions, t);
                }
                return true;
            }
        }

        return false;
    }

    fn in_attack_scan_loop(&self) -> bool {
        let four_rounds_ago = self.round_id - 2;
        let history_entries: Vec<HistoryEntry> = self.history
            .iter()
            .cloned()
            .filter(|he| he.round_id >= four_rounds_ago && he.round_id != self.round_id)
            .collect();
        for entry in history_entries {
            if entry.mode != MODE_ATTACK {
                return false;
            } else {
                let cannon_count = entry.actions
                    .iter()
                    .filter(|ac| ac.action_type == CANNON.to_string())
                    .count();
                if cannon_count > 0 {
                    return false;
                }
            }
        }

        return true;
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

    fn one_bot_attack_strategy(&self, actions: &mut Vec<Action>, target: Option<Pos>) -> bool  {
        println!("one_bot_attack_strategy");
        // If we are evading already let's continue doing that until we're safe
        // Might need to revise this strategy to take risks and be aggressive if all other teams
        // track targets as well as we do
        let mut mode = false;
        let free_bot = self.get_one_bot(&actions);
        println!("Live bot {:?}", free_bot);
        if let Some(bot) = free_bot {
            // If we have a target we should be in attack mode even if we have to evade
            if let Some(t) = target {
                mode = true;
            };

            // If evading we can't really attack
            if self.bots_available_for_attack(&actions) < 1 {
                println!("No bots available for attack");
                return mode;
            }

            // If we have a active target this is pretty easy, shoot at it
            if let Some(t) = target {
                println!("Attacking with bot {:?}", &bot);
                actions.set_action_for(bot.id, CANNON, t.random_spread());
                return true;
            };

            // If not things get more interesting, let's see if we are in attack mode
            let last_round_id = self.round_id - 1;
            let last_round = &self.history.get(&last_round_id);
            if let Some(last_round) = self.history.get(&last_round_id) {
                println!("last_round {:?}", last_round);
                if last_round.mode == MODE_ATTACK.to_string() {
                    if let Some(cannon_action) = last_round.actions
                        .iter().find(|ac| ac.action_type == CANNON.to_string()) {
                        actions.set_action_for(bot.id, RADAR, cannon_action.pos.random_spread());
                    };
                }
            }
        };
        return mode;
    }

    // Get the one free bot (or only alive bot)
    // We should only call this if `bots_available_for_attack` returns 1
    fn get_one_bot(&self, actions: &Vec<Action>) -> Option<Bot> {
        let alive_bots: Vec<Bot> = self.bots
            .iter()
            .cloned()
            .filter(|bot| bot.alive)
            .collect();
        // If only one alive bot return it
        if alive_bots.len() == 1 {
            return Some(alive_bots[0].clone());
        }
        // Otherwise return the first bot not evading
        alive_bots
            .iter()
            .cloned()
            .filter(|bot| {
                if let Some(ac) = actions.iter().find(|ac| ac.bot_id == bot.id) {
                    ac.action_type != MOVE.to_string()
                } else {
                    false
                }
            })
            .find(|bot| bot.alive)
    }

    // Takes the action array and a position to attack and modifies it
    pub fn all_shoot_or_scan(&self, actions: &mut Vec<Action>, target: Pos) {
        println!("All shoot or scan");
        let mut radared: bool = false;
        for bot in &self.bots {
            if bot.alive == true {
                if Some(actions.get_action(bot.id)).unwrap().unwrap().action_type == MOVE.to_string() {
                    // Already on the move, let's keep it that way
                    println!("Already on the move, not changing bot {:?}", bot.id);
                } else if radared == false && self.bots_alive() > 1 {
                    println!("Setting attack radar for bot {:?}", bot.id);
                    actions.set_action_for(bot.id, RADAR, target);
                    radared = true;
                } else {
                    actions.set_action_for(bot.id, CANNON, target.random_spread().clamp(&self.config.field_radius));
                    println!("Setting attack cannon for bot {:?}", bot.id);
                }
            }
        }
    }

    // See if we need this for anything, with the current logic probably not
    // Maybe some edge cases with the one bot strategy?
    #[allow(dead_code)]
    fn get_pos_from_hit_entry(&self, event_entry: &(Event,i16)) -> Option<Pos> {
        let previous_round: i16 = event_entry.1 - 1;
        let mut source: i16 = -1;
        match event_entry.0 {
            Event::Hit(ref ev) => source = ev.source,
            _ => ()
        };
        let cannons = self.history
            .get_actions( CANNON, 20 )
            .iter()
            .cloned()
            .filter(|&(ref ev, ref round_id)| *round_id == previous_round && ev.bot_id == source)
            .collect::<Vec<(Action,i16)>>();
        // Should be guaranteed to have 1 cannon match
        if cannons.len() > 0 {
            return Some(cannons[0].0.pos);
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
                        if self.is_our_bot(ev.bot_id) {
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
