extern crate serde; extern crate serde_json;

use position::Pos;
use defs;
use defs::{Config, Event, Action, ActionsMessage, IncomingEvents };
use defs::Event::*;
use strings::{ ACTIONS, MODE_SCAN, MODE_ATTACK, NOACTION };
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
    #[allow(dead_code)]
    game_map: Vec<Pos>,
    // One entry per game round, could be a bit risky if rounds don't come in order
    history: Vec<HistoryEntry>,
    config: Config,
    logger: Logger,
}

impl Ai {
    fn make_decisions(&mut self) -> (String, Vec<Action>) {
        let mut actions: Vec<Action> = Vec::populate(&self.bots); // NOACTIONS for every live bot

        // Set default mode, mode's are MODE_ATTACK or MODE_SCAN
        // evading is considered something that is up to each bot regardless of mode
        let mut mode = MODE_SCAN.to_string();

        self.logger.log("Making decisions", 1);
        println!("\n---------------------------\nROUND: {:?}\n---------------------------\n", self.round_id);

        // Let each bot evade as needed
        self.evade_if_needed(&mut actions);

        // Attack if we have a target, evading bots will continue evading
        let attacking: bool = self.attack_and_scan_if_target(&mut actions);

        // If not attacking, use non evading bots to scan in a sequence
        if attacking {
            mode = MODE_ATTACK.to_string()
        } else {
            self.scan_with_idle_bots(&mut actions);
        }

        println!("Action are {:?}", actions);
        // Filter out NOACTIONs before sending to server
        return (mode,actions.iter().cloned().filter(|ac| ac.action_type != NOACTION.to_string()).collect());
    }

    pub fn new(start: &defs::Start) -> Ai {
        // TODO: separate into smaller functions to do set up
        let mut radar: radar::Radar = radar::Radar::new();
        let radar_positions = &radar.get_radar_positions(&start.config);
        let mut game_map: Vec<Pos> = Pos { x: 0, y: 0 }.neighbors(&start.config.field_radius);
        game_map.push(Pos { x: 0, y: 0 });

        return Ai {
            bots: gen_bots(start),
            round_id: -1,
            radar_positions: (0, radar_positions.clone()),
            game_map: game_map.clone(),
            history: Vec::new(),
            config: start.config.clone(),
            logger: Logger::new(),
        };
    }

    fn bots_alive(&self) -> usize {
        self.bots.iter().filter(|bot| bot.alive ).count()
    }

    fn is_our_bot(&self, bot_id: i16) -> bool {
        match self.get_bot(bot_id) {
            Some(_) => return true,
            _ => return false
        }
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

    // Purpose: go through events and update our state so it's up to date for decisionmaking later
    fn update_state(&mut self, events: &Vec<Event>) {
        let mut log: Vec<(String, usize)> = Vec::new();
        for event in events {
            match *event {
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
                    log.push((format!("RadarEcho enemy on {}", ev.pos), 2));
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

        // Add events to history
        self.history.add_events(&self.round_id, &events);

        // Get mode and actions for the round and add those to history too
        let (mode,actions) = self.make_decisions();
        self.history.set_mode(&mode);
        self.history.add_actions(&self.round_id, &actions);

        return self.make_actions_message(actions);
    }

    fn get_bot(&self, id: i16) -> Option<&Bot> {
        return self.bots.iter().find(|bot|bot.id == id);
    }

    fn get_bot_mut(&mut self, id: i16) -> Option<&mut Bot> {
        return self.bots.iter_mut().find(|bot|bot.id == id);
    }
}

fn gen_bots(start: &defs::Start) -> Vec<Bot> {
    return start.you.bots.iter().map(Bot::new).collect();
}


