use defs::{ Action, Event, HitEvent, DieEvent };
use strings::{ CANNON, RADAR, MOVE, SEE, RADARECHO, DIE, HIT };
use position::Pos;
use rand;
use rand::Rng;
use ai::*;
use lists::*;

impl Ai {
    pub fn attack_and_scan_if_target(&self, mut actions: &mut Vec<Action>) -> bool {
        // Separate the logic needed if we only have one bot left
        if self.bots_alive() == 1 {
            return self.one_bot_attack_strategy(&mut actions);
        }

        let mut target: Option<Pos> = None;
        println!("Bots available for attack {:?}", self.bots_available_for_attack(&actions));

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
        if let Some(t) = target {
            self.all_shoot_or_scan(&mut actions, t);
            return true;
        }

        // Let's see when the last time was we knew about an enemy position
        println!("Last enemy position {:?}", self.history.get_last_enemy_position());

        // Next let's see if we hit something that we didn't radar
        let hit_events: Vec<Event> = self.history.get_events_for_round( HIT, self.round_id )
            .iter()
            .cloned()
            .filter(|hit| {
                match hit {
                    &Event::Hit(ref ev) => !self.is_our_bot(ev.bot_id),
                    _ => false,
                }
            })
            .collect();
        println!("Hit events {:?}", self.history.get_events(HIT, 1));
        println!("Filtered hit events {:?}", hit_events);
        for hit in hit_events {
            match hit {
                Event::Hit(ref ev) => {
                    let our_bot = ev.source;
                    if let Some(action) = self.history.get_action_for_bot(&our_bot, &self.round_id) {
                        println!("We hit something at {:?}", action.pos);
                        if self.bots_available_for_attack(&actions) > 1 {
                            self.all_shoot_or_scan(&mut actions, action.pos);
                            return true;
                        } else {
                            self.all_shoot_or_scan(&mut actions, action.pos);
                            return true;
                        }
                    }
                },
                _ => ()
            }
        }

        // Next, let's see if we were attacking last round


        // were we attacking previously?
        // let mut killed_target = false;
        // if let Some(pos) = self.last_attack_pos() {
        //     println!("Last attack Pos: {:?}", pos);
        //     if !self.target_still_there(pos) {
        //         println!("Killed target {:?}", pos);
        //         killed_target = true;
        //     }
        // }

        if let Some(t) = target {
            self.all_shoot_or_scan(&mut actions, t);
        }
        return false;
    }

    // Returns the number of bots that are alive and not evading
    pub fn bots_available_for_attack(&self, actions: &Vec<Action>) -> usize {
        self.bots
            .iter()
            .filter(|bot| bot.alive && {
                if let Some(ac) = actions.iter().find(|ac| ac.bot_id == bot.id) {
                    println!("Action for bot {:?}", ac);
                    ac.action_type != MOVE.to_string()
                } else {
                    println!("No action for bot {:?}", bot);
                    false
                }
            })
            .count()
    }

    fn one_bot_attack_strategy(&self, actions: &mut Vec<Action>) -> bool  {
        // Exit early if we don't have any bots available for attack
        if self.bots_available_for_attack(&actions) < 1 {
            return false;
        }

        return false;
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
                    println!("Setting attack radar for bot {:?} {}", bot.id, radared);
                    actions.set_action_for(bot.id, RADAR, target);
                    radared = true;
                } else {
                    actions.set_action_for(bot.id, CANNON, target.random_spread().clamp(&self.config.field_radius));
                    println!("Setting attack cannon for bot {:?}", bot.id);
                }
            }
        }
    }

    pub fn all_shoot_at_action(&self, actions: &mut Vec<Action>, target: &Pos) {
        self.bots
            .iter()
            // TODO: Maybe add shuffle triangle here?
            // TODO: Random shooting at middle
            .zip(Pos::triangle_smart(target).iter())
            .map(|(bot, pos)| {
                actions.set_action_for(bot.id, CANNON, *pos);
                Action {
                    bot_id: bot.id,
                    action_type: CANNON.to_string(),
                    pos: *pos,
            }}).count();
    }

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

    fn target_still_there(&self, last_pos: Pos) -> bool {
        let echoes = self.history.get_events( RADARECHO, 1 );
        println!("target_still_there echoes {:?}", echoes);
        if echoes.len() > 0 {
            match echoes[0].0 {
                Event::Echo(ref ev) => {
                    if ev.pos.distance(last_pos) <= 3 {
                        return true;
                    }
                },
                _ => ()
            }
        }
        return false;
    }

    fn last_attack_pos(&self) -> Option<Pos> {
        if self.history.get_actions( CANNON, 3).len() > 0 {
            let radar = self.history.get_actions( RADAR, 2);
            if radar.len() == 1 {
                return Some(radar[0].0.pos.clone());

            }
        }
        return None;
    }

    fn get_possible_kill_shot(&self) -> Option<Action> {
        let last_round: i16 = self.round_id as i16 - 1;
        let possible_die_event = self.get_possible_die_event();
        let die_event = match possible_die_event {
            Some(ev) => Some(ev),
            None => return None,
        };

        println!("Die event {:?}", die_event);

        let hit_events: Vec<Option<HitEvent>> = self.history
            .get_events( HIT, 3 )
            .iter()
            .cloned()
            // .filter(|&(ref ev, ref round_id)| *round_id == last_round)
            .map(|(ref ev, ref round_id)| match ev {
                &Event::Hit(ref e) => Some(e.clone()),
                _ => None,
            })
            .collect();
        println!("Hit events {:?}", hit_events);

        let mut hit: Option<HitEvent> = None;
        // Find at least one hit matching the dead bot
        for hits in hit_events {
            match hits {
                Some(ev) => if ev.bot_id == die_event.unwrap().bot_id {
                    hit = Some(ev);
                },
                None => ()
            }
        }

        println!("Kill shot hit {:?}", hit);
        match hit {
            Some(hit_ev) => {
                let actions = self.history
                    .get_actions( CANNON, 1 );
                for ac in actions {
                    if ac.0.bot_id == hit_ev.bot_id {
                        return Some(ac.0.clone());
                    }
                }
                return None;
            },
            None => return None,
        }

        return None;
    }

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
