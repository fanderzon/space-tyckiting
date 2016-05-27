extern crate serde; extern crate serde_json;

use defs;
use defs::Event::*;
use defs::{Config, Event, Action, ActionsMessage, IncomingEvents };
use tribool::Tribool;
use tribool::Tribool::*;
use position::Pos;
use strings::{ ACTIONS, NOACTION, CANNON };
use lists::*;
use lists::ActionMode::*;
use ai::bot::Bot;
use log::Logger;

mod radar;
mod evade;
mod scan;
mod attack;
pub mod bot;

pub struct Ai {
    bots: Vec<Bot>,
    round_id: i16,
    radar_positions: (i16, Vec<Pos>),
    history: Vec<HistoryEntry>,
    asteroids: Vec<(Pos, bool)>,
    config: Config,
    logger: Logger,
}

impl Ai {
    fn make_decisions(&mut self) -> (Decision, Vec<Action>) {
        let mut actions: Vec<Action> = Vec::populate(&self.bots); // NOACTIONS for every live bot
        let mut decision = Decision::with_defaults();

        self.logger.log("Decisions", 1);
        println!("\n---------------------------\nROUND: {:?}\n---------------------------\n", self.round_id);

        // Let each bot evade as needed
        self.evade_if_needed(&mut actions);

        // Attack if we have a target, evading bots will continue evading
        self.aggressive_attack_strategy(&mut actions, &mut decision);

        // If not attacking, use non evading bots to scan in a sequence
        match decision.mode {
            Attack => (),
            _ => self.scan_with_idle_bots(&mut actions, &mut decision),
        }
        self.logger.log(&format!("Mode: {}", decision.mode), 2);

        println!("Action are {:?}", actions);
        // Filter out NOACTIONs before sending to server
        return (decision, actions.iter().cloned().filter(|ac| ac.action_type != NOACTION.to_string()).collect());
    }

    pub fn new(start: &defs::Start) -> Ai {
        // TODO: separate into smaller functions to do set up
        let mut radar: radar::Radar = radar::Radar::new();
        let radar_positions = &radar.get_radar_positions(&start.config);

        return Ai {
            bots: gen_bots(start),
            round_id: -1,
            radar_positions: (0, radar_positions.clone()),
            history: Vec::new(),
            asteroids: Vec::new(),
            config: start.config.clone(),
            logger: Logger::new(),
        };
    }


    // TODO: This does not actually need to be mutable
    fn make_actions_message(&self, mut actions: Vec<Action>) -> ActionsMessage {
        actions.reverse();  // Apparently by "latest", futurice means "first in array". So we need
                            // to put our "latest" actions "first".
        return ActionsMessage {
            event_type: ACTIONS.to_string(),
            round_id: self.round_id,
            actions: actions,
        };
    }

    fn is_pos_in_cannon_actions(&self, pos: Pos, cannon_positions: &Vec<Pos>) -> bool {
        cannon_positions.iter().find(
            |cp| cp.distance(pos) <= self.config.cannon
            ).is_some()
    }

    // Logic:
    // If pos is within shoot radius of any cannon action,
    // AND there is no hit event at that position, we have an asteroid.
    // Otherwise we can't be sure, so return false.
    // For this logic to work, we must have shot while radaring.
    // Maybe we should ensure this somehow later?
    // We do not want any false positives here...
    fn is_echo_an_asteroid(&self, pos: Pos, hit_events: &Vec<Event>) -> Tribool {
        // Check if this is already a recorded asteroid to save some work
        if self.asteroids.is_asteroid(pos) {
            return Yes;
        }

        let we_sure;
        let cannon_positions = match self.bots_alive() {
            1 => {
                // If we only have one bot, we couldn't shoot and fire at once.
                we_sure = Maybe;
                self.history
                    .get_actions_for_round( CANNON, self.round_id - 2 )
                    .iter()
                    .map(|ac|ac.pos)
                    .collect()
            }
            _ => {
                we_sure = Yes;
                self.history
                    .get_actions_for_round( CANNON, self.round_id - 1 )
                    .iter()
                    .map(|ac|ac.pos)
                    .collect()
            }
        };

        // First we check if the position matches any cannon actions
        // Otherwise we can't draw any conclusions
        if self.is_pos_in_cannon_actions(pos, &cannon_positions) {
            // Next see if we can find a hit event on that position
            if self.is_in_hit_events(pos, hit_events) {
                return No;
            } else {
                return we_sure;
            }
            // If we shot at pos but didn't get a hit event, it's an asteroid.
        } else {
            // We didn't shoot at it so we have no way of telling if it's an asteroid yet
            return No;
        }
    }

    fn is_in_hit_events(&self, pos: Pos, hit_events: &Vec<Event>) -> bool {
        return hit_events
            .iter()
            .filter_map(|ev| self.get_pos_from_hit(&ev, self.round_id))
            .find(|hit_pos| hit_pos.distance(pos) <= self.config.cannon)
            .is_some();
    }

    fn filter_asteroids_from_events(&mut self, events: &Vec<Event>) -> Vec<Event> {
        let hit_events_this_round = events
            .iter()
            .cloned()
            .filter(|event| {
                match *event {
                    Hit(_) => true,
                    _ => false,
                }
            })
            .collect::<Vec<_>>();

        let mut events: Vec<Event> = events.to_vec();
        events.retain(|event| { match *event {
            Echo(ref ev) => {
                let is_asteroid_now = self.is_echo_an_asteroid(ev.pos, &hit_events_this_round );

                if Yes == is_asteroid_now && !self.asteroids.is_asteroid(ev.pos) {
                    self.logger.log(&format!("Recorded an asteroid at {}.", ev.pos), 2);
                    self.asteroids.register(ev.pos);
                } else if Maybe == is_asteroid_now {
                    if self.asteroids.is_maybe_asteroid(ev.pos) {
                        println!("Two maybes on {:?}, it's probbly an asteroid.", ev.pos);
                        self.logger.log(&format!("Recorded an asteroid at {} because of 2 maybes.", ev.pos), 2);
                        self.asteroids.register(ev.pos);
                    } else {
                        println!("Echo {:?} might be an asteroid", ev.pos);
                        self.logger.log(&format!("Recording that there might be an asteroid at {}.", ev.pos), 2);
                        self.asteroids.register_maybe(ev.pos);
                    }
                }

                Yes != is_asteroid_now
            }
            _ => true
        }});
        return events;
    }

    // Purpose: go through events and update our state so it's up to date for decisionmaking later
    fn update_state(&mut self, events: &Vec<Event>) {
        self.logger.log("Events:", 1);
        let mut log: Vec<(String, usize)> = Vec::new();

        for event in events {
            match *event {
                Hit(ref ev) => {
                    let mut log_msg = String::from("Hit ");
                    if self.is_our_bot(ev.bot_id) {
                        log_msg.push_str(&format!("own bot {}", ev.bot_id));
                    } else {
                        log_msg.push_str(&format!("enemy bot {}", ev.bot_id));
                    }

                    log_msg.push_str(" by ");

                    if self.is_our_bot(ev.source) {
                        log_msg.push_str(&format!("own bot {}", ev.source));
                    } else {
                        log_msg.push_str(&format!("enemy bot {}", ev.source));
                    }

                    log.push((log_msg, 2));
                }
                Die(ref ev) => {
                    let bot_opt = self.get_bot_mut(ev.bot_id);
                    if let Some(bot) = bot_opt {
                        bot.alive = false;
                        log.push((format!("Die own bot {}", bot.id), 2));
                    } else {
                        log.push((format!("Die enemy bot {}", ev.bot_id), 2));
                    }
                }
                See(ref ev) => {
                    log.push((format!("See enemy on {:?}", ev.pos), 2));
                }
                SeeAsteroid(ref ev) => {
                    println!("SeeAsteroid at {}", ev.pos);
                    log.push((format!("SeeAsteroid at {}", ev.pos), 2));
                    self.asteroids.register(ev.pos);
                }
                Echo(ref ev) => {
                    //TODO!!!
                    log.push((format!("Echo enemy at {}", ev.pos), 2));
                }
                Damaged(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    bot.hp -= ev.damage;
                    log.push((format!("Bot {} damaged {} hp, {} hp left.", ev.bot_id, ev.damage, bot.hp), 2));
                }
                Move(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    let oldpos = bot.pos;
                    bot.pos = ev.pos;
                    log.push((format!("Move own bot {} from {} to {}", bot.id, oldpos, bot.pos), 2));
                }
                Detected(ref ev) => {
                    let bot = self.get_bot(ev.bot_id).expect("No bot on our team with this id wtf?");
                    log.push((format!("Was Detected own bot {} on pos {}", bot.id, bot.pos), 2));
                }
                Noaction(_) => {
                    //TODO: Maybe we can use the knowledge that a bot is sleeping? To exploit bugs
                    //in enemy code ;)
                }
                _ => {}
            }
        }

        self.logger.write_q(&log);
    }

    pub fn handle_message(&mut self, events_json: IncomingEvents) -> ActionsMessage {
        self.round_id = events_json.round_id;
        self.logger.log(&format!("round {}", self.round_id), 0);

        let events = events_json.events.iter().map(defs::parse_event).collect();
        let events: Vec<Event> = self.filter_asteroids_from_events(&events);
        self.update_state(&events);

        // Add events to history after filtering out asteroids
        self.history.add_events(self.round_id, &events);

        // Get mode and actions for the round and add those to history too
        let (decision,actions) = self.make_decisions();

        self.history.add_actions(self.round_id, &actions);
        self.history.set_decision(self.round_id, decision);

        self.logger.log(&actions.render(), 2);
        return self.make_actions_message(actions);
    }

    fn get_bot(&self, id: i16) -> Option<&Bot> {
        return self.bots.iter().find(|bot|bot.id == id);
    }

    fn get_bot_mut(&mut self, id: i16) -> Option<&mut Bot> {
        return self.bots.iter_mut().find(|bot|bot.id == id);
    }

    fn bots_alive(&self) -> usize {
        self.bots.iter().filter(|bot| bot.alive ).count()
    }

    pub fn get_live_bots(&self) -> Vec<Bot> {
        self.bots.iter().filter(|bot| bot.alive ).cloned().collect::<Vec<_>>()
    }

    pub fn is_our_bot(&self, bot_id: i16) -> bool {
        match self.get_bot(bot_id) {
            Some(_) => true,
            None => false,
        }
    }
}

fn gen_bots(start: &defs::Start) -> Vec<Bot> {
    return start.you.bots.iter().map(Bot::new).collect();
}
