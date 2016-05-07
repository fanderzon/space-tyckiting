extern crate serde; extern crate serde_json;

use std::str::from_utf8;
use websocket::Message;
use websocket::message::Type;
use position::Pos;
use util;
use defs;
use defs::{Config, Start, Event, Action, ActionsMessage, IncomingMessage, IncomingEvents, SomeEvent};
use defs::Event::*;
use strings::{ ACTIONS, CANNON, END, EVENTS, RADAR, MOVE, RADARECHO };
use lists::*;

mod radar;
mod evade;
mod scan;
mod attack;

pub struct Ai {
    bots: Vec<Bot>,
    round_id: i16,
    radar_positions: Vec<Pos>,
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
    fn make_decisions(&self, events: Vec<Event>) -> Vec<Action> {
        // Populate an actions vector with a no action for each bot
        let mut actions: Vec<Action> = Vec::populate(&self.bots);
        let mut alive_bots = self.bots.iter().filter(|b| b.alive).map(|b| b.clone()).collect::<Vec<Bot>>();

        // Add random radar actions as default
        self.random_radars_action(&mut actions);
        println!("\n---------------------------\nROUND: {:?}\n---------------------------\n", self.round_id);

        // Try getting history events
        let historic_echoes = self.history.get_events( RADARECHO, 2 );
        println!("Historic echo events {:?}", historic_echoes);

        for event in events {
            match event {
                Damaged(ref ev) => {
                    println!("Evading on bot {}", ev.bot_id);
                    actions.set_action_for(ev.bot_id, MOVE, self.evade_pos(self.get_bot(ev.bot_id).unwrap()));
                }
                Detected(ref ev) => {
                    println!("Evading on bot {}", ev.bot_id);
                    actions.set_action_for(ev.bot_id, MOVE, self.evade_pos(self.get_bot(ev.bot_id).unwrap()));
                }
                Hit (ref ev) => {
                    let target = ev.bot_id;
                    match self.get_bot(target) {
                        None => {
                            // It wasn't out bot that was damaged, find the last action of the source
                        },
                        _ => {}
                    }
                },
                Echo(ref ev) => {
                    println!("Got echo, gonna shoot at it!");
                    self.all_shoot_or_scan(&mut actions, ev.pos);
                    // println!("Actions from echo {:?}", actions);
                }
                See(ref ev) => {
                    self.all_shoot_or_scan(&mut actions, ev.pos);
                    println!("Saw something, gonna shoot at it!");
                }
                _ => {}
            }
        }

        println!("Action are {:?}", actions);
        return actions;
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
            radar_positions: radar_positions.clone(),
            game_map: game_map.clone(),
            history: Vec::new(),
            config: start.config.clone(),
        };
    }

    fn bots_alive(&self) -> usize {
        self.bots.iter().filter(|bot| bot.alive ).count()
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
                See(ref ev) => {

                }
                Echo(ref ev) => {

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
                println!("It's text!");

                let pl = from_utf8(&message.payload).unwrap();
                let message_json: IncomingMessage = serde_json::from_str(&pl).unwrap();

                match message_json.event_type.as_ref() {
                    EVENTS => {
                        println!("Got som events!");
                        let events_json: IncomingEvents = serde_json::from_str(&pl).unwrap();
                        self.round_id = events_json.round_id;
                        let events = events_json.events.iter().map(defs::parse_event).collect();
                        self.update_state(&events);
                        self.history.add_events(&self.round_id, &events);
                        println!("Loggin events {:?}", &events);
                        println!("Logging History {:?}", &self.history);
                        return Ok(self.make_actions_message(self.make_decisions(events)));
                    }
                    END => {
                        println!("Got end message, we're ending!");
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

    fn clone(&self) -> Bot {
        Bot {
            id: self.id,
            name: self.name.to_string(),
            alive: self.alive,
            pos: self.pos,
            hp: self.hp
        }
    }
}
