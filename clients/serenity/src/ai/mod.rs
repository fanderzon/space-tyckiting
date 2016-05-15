extern crate serde; extern crate serde_json;

use position::Pos;
use defs;
use defs::{Config, Event, Action, ActionsMessage, IncomingEvents };
use defs::Event::*;
use strings::{ ACTIONS, MODE_SCAN, MODE_ATTACK, NOACTION, CANNON, HIT };
use lists::*;
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
    asteroids: Vec<Pos>,
    config: Config,
    logger: Logger,
}

impl Ai {
    fn make_decisions(&mut self) -> (String, Vec<Action>) {
        let mut actions: Vec<Action> = Vec::populate(&self.bots); // NOACTIONS for every live bot

        // Set default mode, mode's are MODE_ATTACK or MODE_SCAN
        // evading is considered something that is up to each bot regardless of mode
        let mut mode = MODE_SCAN.to_string();

        self.logger.log("Decisions", 1);
        println!("\n---------------------------\nROUND: {:?}\n---------------------------\n", self.round_id);

        // Let each bot evade as needed
        self.evade_if_needed(&mut actions);

        // Attack if we have a target, evading bots will continue evading
        let attacking: bool = self.aggressive_attack_strategy(&mut actions);

        // If not attacking, use non evading bots to scan in a sequence
        if attacking {
            mode = MODE_ATTACK.to_string();
        } else {
            self.scan_with_idle_bots(&mut actions);
        }
        self.logger.log(&format!("Mode: {}", mode), 2);

        println!("Action are {:?}", actions);
        // Filter out NOACTIONs before sending to server
        return (mode,actions.iter().cloned().filter(|ac| ac.action_type != NOACTION.to_string()).collect());
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
        if cannon_positions.iter().filter(|cp| cp.distance(pos) <= 1).count() > 0 {
            true
        } else {
            false
        }
    }

    // So the logic here is:
    // If the supplied position is within 1 distance of any cannon action
    // AND there is no hit event at that position we have an asteroid and return true
    // Otherwise return false
    fn is_echo_an_asteroid(&self, pos: Pos, cannon_positions: &Vec<Pos>, hit_events: &Vec<Event>) -> bool {
        // If we have already recorded all asteroids on the board don't even bother to check
        if self.asteroids.len() as i16 == self.config.asteroids {
            return false;
        }

        // First we check if the position matches any cannon actions
        // Otherwise we can't draw any conclusions
        if self.is_pos_in_cannon_actions(pos, &cannon_positions) {
            // Next see if we can find a hit event on that position
            hit_events.iter().cloned().map(|ref event| {
                if let Some(p) = self.get_pos_from_hit(&event, &self.round_id) {
                    if p.distance(pos) == 0 {
                        return false;
                    }
                }
                true
            }).count();
            // The position was in cannon actions but didn't match any hits.... ASTEROID!!!
            return true;
        } else {
            // We didn't shoot at it so we have no way of telling if it's an asteroid yet
            return false;
        }
    }

    pub fn is_pos_a_recorded_asteroid(&self, pos: Pos) -> bool {
        if let Some(_) = self.asteroids
            .iter()
            .find(|asteroid_pos| asteroid_pos.distance(pos) == 0)
        {
            return true;
        } else {
            return false;
        }
    }

    fn filter_asteroids_from_events(&self, events: &Vec<Event>) -> Vec<Event> {
        events
            .iter()
            .cloned()
            .filter(|event| {
                match *event {
                    Echo(ref ev) => {
                        if self.asteroids.iter().filter(|pos| pos.distance(ev.pos) == 0).count() > 0 {
                            false
                        } else {
                            true
                        }
                    }
                    _ => true
                }
            })
            .collect::<Vec<_>>()
    }

    // Purpose: go through events and update our state so it's up to date for decisionmaking later
    fn update_state(&mut self, events: &Vec<Event>) {
        self.logger.log("Events:", 1);
        let mut log: Vec<(String, usize)> = Vec::new();
        let last_round_cannon_positions: Vec<Pos> = self.history
            .get_actions_for_round( CANNON, self.round_id - 1 )
            .iter()
            .cloned()
            .map(|ac|ac.pos)
            .collect();
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

        let mut neg_probes: Vec<Pos> = Vec::new();

        for event in events {
            match *event {
                Hit(ref ev) => {
                    // let probe = self.probing.into_iter().find(|&(id, pos)| ev.source == id);
                    // if let Some((_, pos)) = probe {
                    //     neg_probes.push(pos);
                    // }
                }
                Die(ref ev) => {
                    let bot_opt = self.get_bot_mut(ev.bot_id);
                    if let Some(bot) = bot_opt {
                        bot.alive = false;
                        log.push((format!("Die own bot {}", bot.id), 2));
                    } else {
                        // TODO: Enemy bot died, this should be recorded somehow.
                    }
                }
                See(ref ev) => {
                    log.push((format!("See enemy on {:?}", ev.pos), 2));
                }
                Echo(ref ev) => {
                    log.push((format!("RadarEcho enemy/asteroid on {}", ev.pos), 2));
                    if self.is_echo_an_asteroid(ev.pos, &last_round_cannon_positions, &hit_events_this_round) {
                        log.push((format!("Recorded an asteroid at {}.", ev.pos), 2));
                        self.asteroids.push(ev.pos);
                    }
                }
                Damaged(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    bot.hp -= ev.damage;
                    log.push((format!("Bot {} damaged {} hp, {} hp left.", ev.bot_id, ev.damage, bot.hp), 2));
                }
                Move(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    let oldpos = bot.pos.clone();
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
        let log_msg = format!("round {}", self.round_id);
        self.logger.log(&log_msg, 0);
        let events = events_json.events.iter().map(defs::parse_event).collect();
        self.update_state(&events);

        // Add events to history after filtering out asteroids
        let events_without_asteroids: Vec<Event> = self.filter_asteroids_from_events(&events);
        self.history.add_events(&self.round_id, &events_without_asteroids);

        // Get mode and actions for the round and add those to history too
        let (mode,actions) = self.make_decisions();

        self.history.add_actions(&self.round_id, &actions);
        self.history.set_mode(&self.round_id, &mode);

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
