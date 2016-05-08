extern crate serde; extern crate serde_json;

use std::str::from_utf8;
use websocket::Message;
use websocket::message::Type;
use position::Pos;
use defs;
use defs::{Config, Event, Action, ActionsMessage, IncomingMessage, IncomingEvents, IncomingEnd };
use defs::Event::*;
use strings::{ ACTIONS, END, EVENTS, MODE_SCAN, MODE_ATTACK };
use lists::*;

mod radar;
mod evade;
mod scan;
mod attack;

pub struct Ai {
    bots: Vec<Bot>,
    round_id: i16,
    radar_positions: (i16, Vec<Pos>),
    #[allow(dead_code)]
    game_map: Vec<Pos>,
    // One entry per game round, could be a bit risky if rounds don't come in order
    history: Vec<HistoryEntry>,
    config: Config,
}

#[derive(PartialEq)]
pub enum NoAction {
    Ignore,
    Exit,
}

impl Ai {
    fn make_decisions(&mut self) -> (String, Vec<Action>) {
        // Populate an actions vector with a default action for each bot
        let mut actions: Vec<Action> = Vec::populate(&self.bots, &self.radar_positions.1);
        let mut mode = MODE_SCAN.to_string();
        println!("\n---------------------------\nROUND: {:?}\n---------------------------\n", self.round_id);

        // Add random radar actions as default
        self.random_radars_action(&mut actions);

        // Evade if needed
        self.evade_if_needed(&mut actions);

        // Attack if we have a target
        let attacking: bool = self.attack_and_scan_if_target(&mut actions);

        // If not attacking, use non evading bots to scan in a sequence
        if attacking {
            mode = MODE_ATTACK.to_string()
        } else {
            self.scan_with_idle_bots(&mut actions);
        }

        println!("Action are {:?}", actions);
        return (mode,actions);
    }

    pub fn new(start: &defs::Start) -> Ai {
        // TODO: separate into smaller functions to do set up
        let mut radar: radar::Radar = radar::Radar::new();
        let radar_positions = &radar.get_radar_positions(&start.config);
        let mut game_map: Vec<Pos> = Pos { x: 0, y: 0 }.neighbors(&start.config.field_radius);
        game_map.push(Pos { x: 0, y: 0 });

        return Ai {
            bots: start.you.bots.iter().map(Bot::new).collect(),
            round_id: -1,
            radar_positions: (0, radar_positions.clone()),
            game_map: game_map.clone(),
            history: Vec::new(),
            config: start.config.clone(),
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
        for event in events {
            match *event {
                Die(ref ev) => {
                    if let Some(bot) = self.get_bot_mut(ev.bot_id) {
                        bot.alive = false;
                    } else {
                        // TODO: Enemy bot died, this should be recorded somehow.
                    }
                }
                See(_) => {

                }
                Echo(_) => {

                }
                Damaged(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    bot.hp -= ev.damage;
                }
                Move(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    bot.pos = ev.pos;
                }
                Noaction(_) => {
                    //TODO: Maybe we can use the knowledge that a bot is sleeping? To exploit bugs
                    //in enemy code ;)
                }
                _ => {}
            }
        }
    }

    pub fn handle_message(&mut self, message: Message) -> Result<ActionsMessage, NoAction> {
        match message.opcode {
            Type::Text => {
                let pl = from_utf8(&message.payload).unwrap();
                let message_json: IncomingMessage = serde_json::from_str(&pl).unwrap();

                match message_json.event_type.as_ref() {
                    EVENTS => {
                        let events_json: IncomingEvents = serde_json::from_str(&pl).unwrap();
                        self.round_id = events_json.round_id;
                        let events = events_json.events.iter().map(defs::parse_event).collect();
                        self.update_state(&events);
                        self.history.add_events(&self.round_id, &events);
                        let (mode,actions) = self.make_decisions();
                        self.history.set_mode(&mode);
                        self.history.add_actions(&self.round_id, &actions);
                        return Ok(self.make_actions_message(actions));
                    }
                    END => {
                        let end: IncomingEnd = serde_json::from_str(&pl).unwrap();
                        println!("Game ended!");
                        if end.you.team_id == end.winner_team_id {
                            println!("WE WON!!!");
                        } else {
                            println!("WE DIDN'T WIN!!!");
                        }
                        return Err(NoAction::Exit);
                    }
                    ev => {
                        println!("Got unrecognized event type {}, ignoring.", ev);
                    }
                }
            }
            _ => {
                println!("Got a weird non-text message from server, ignoring.");
            }
        }
        return Err(NoAction::Ignore);
    }

    fn get_bot(&self, id: i16) -> Option<&Bot> {
        return self.bots.iter().find(|bot|bot.id == id);
    }

    fn get_bot_mut(&mut self, id: i16) -> Option<&mut Bot> {
        return self.bots.iter_mut().find(|bot|bot.id == id);
    }
}

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Bot {
    pub id: i16,
    pub name: String,
    pub alive: bool,
    pub pos: Pos,
    pub hp: i16,
}

impl Bot {
    fn new(def: &defs::Bot) -> Bot {
        return Bot {
            id: def.bot_id,
            name: def.name.to_owned(),
            alive: def.alive,
            pos: def.pos.unwrap(),
            hp: def.hp.unwrap(),
        };
    }
}
