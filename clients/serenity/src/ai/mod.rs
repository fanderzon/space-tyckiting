extern crate serde; extern crate serde_json;

use std::str::from_utf8;
use util;
use defs;
use defs::{ActionsMessage, IncomingMessage, IncomingEvents};
use websocket::Message;
use websocket::message::Type;
use position::Pos;

pub struct Ai {
    bots: Vec<Bot>,
}

#[derive(PartialEq)]
pub enum NoAction {
    Ignore,
    Exit,
}

impl Ai {
    pub fn new(start: &defs::Start) -> Ai {
        return Ai { bots: start.you.bots.iter().map(Bot::new).collect() };
    }
    pub fn handle_message(&mut self, message: Message) -> Result<ActionsMessage, NoAction> {
        match message.opcode {
            Type::Text => {
                println!("It's text!");

                let pl = from_utf8(&message.payload).unwrap();
                let message_json: IncomingMessage = serde_json::from_str(&pl).unwrap();

                match message_json.event_type.as_ref() {
                    "events" => {
                        println!("Got som events!");
                        let event_json: IncomingEvents = serde_json::from_str(&pl).unwrap();
                        return Ok(self.make_decisions(&event_json));
                    }
                    "end" => {
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

    fn make_decisions(&self, events: &defs::IncomingEvents) -> defs::ActionsMessage {
        let stupid_actions = defs::ActionsMessage {
            event_type: "actions".to_string(),
            round_id: events.round_id,
            actions: self.bots.iter().map(|bot| defs::Action{
                bot_id: bot.id,
                action_type: "radar".to_string(),
                pos: util::get_random_pos()
            }).collect(),
        };
        return stupid_actions;
    }
}

pub struct Bot {
    id: i16,
    name: String,
    alive: bool,
    pos: Pos,
    hp: i16,
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

