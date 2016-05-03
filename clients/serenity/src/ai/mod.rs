extern crate serde; extern crate serde_json;

use std::str::from_utf8;
use websocket::Message;
use websocket::message::Type;
use position::Pos;
use util;
use defs;
use defs::{Start, Action, ActionsMessage, IncomingMessage, IncomingEvents, Event, SomeEvent};
use strings::{ ACTIONS, CANNON, END, EVENTS, RADAR };

mod radar;

pub struct Ai {
    bots: Vec<Bot>,
    round_id: i16,
    radar_positions: Vec<Pos>,
    game_map: Vec<Pos>,
    // One entry per game round, could be a bit risky if rounds don't come in order
    history: Vec<HistoryEntry>,
}

#[derive(PartialEq)]
pub enum NoAction {
    Ignore,
    Exit,
}

impl Ai {
    fn make_decisions(&self, events: &Vec<defs::Event>) -> Vec<Action> {
        for event in events {
            match event {
                _ => {}
            }
        }
        // TODO: Replace with proper logic
        let random_num = util::get_rand_range(0, 2);
        match random_num {
            0 => self.all_shoot_at_action(&util::get_random_pos(&self.game_map)),
            _ => self.random_radars_action(&self.radar_positions)
        }
    }

    pub fn new(start: &defs::Start) -> Ai {
        // TODO: separate into smaller functions to do set up
        let mut radar: radar::Radar = radar::Radar::new();
        let radar_positions = &radar.get_radar_positions(&start.config);
        let mut game_map: Vec<Pos> = Pos { x: 0, y: 0 }.neighbours(&start.config.field_radius);
        game_map.push(Pos { x: 0, y: 0 });

        return Ai {
            bots: start.you.bots.iter().map(Bot::new).collect(),
            round_id: -1,
            radar_positions: radar_positions.clone(),
            game_map: game_map.clone(),
            history: Vec::new(),
        };
    }

    fn all_shoot_at_action(&self, target: &Pos) -> Vec<Action> {
        return self.bots
            .iter()
            // TODO: Maybe add shuffle triangle here?
            // TODO: Random shooting at middle
            .zip(Pos::triangle_down(target).iter())
            .map(|(bot, pos)| Action {
                bot_id: bot.id,
                action_type: CANNON.to_string(),
                pos: *pos,
            }).collect();
    }

    fn random_radars_action(&self) -> Vec<Action> {
        return self.bots.iter().map(|bot| Action {
            bot_id: bot.id,
            action_type: RADAR.to_string(),
            pos: util::get_random_pos(&self.radar_positions)
        }).collect();
    }

    fn make_actions_message(&self, actions: Vec<Action>) -> ActionsMessage {
        return ActionsMessage {
            event_type: ACTIONS.to_string(),
            round_id: self.round_id,
            actions: actions,
        };
    }

    fn parse_events(&self, events: &Vec<SomeEvent>) -> Vec<Event> {
        events
            .iter()
            .map(|ev| defs::parse_event(&ev))
            .collect::<Vec<Event>>()
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
                        return Ok(self.make_actions_message(self.make_decisions(&events)));
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

#[derive(Debug)]
pub struct HistoryEntry {
    round_id: i16,
    events: Vec<Event>
}
