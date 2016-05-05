extern crate serde; extern crate serde_json;

use std::str::from_utf8;
use websocket::Message;
use websocket::message::Type;
use position::Pos;
use util;
use defs;
use defs::{Config, Start, Event, Action, ActionsMessage, IncomingMessage, IncomingEvents, SomeEvent};
use defs::Event::*;
use strings::{ ACTIONS, CANNON, END, EVENTS, RADAR, MOVE };
use lists::*;

mod radar;
mod evade;

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
    fn make_decisions(&self) -> Vec<Action> {
        // Populate an actions vector with a no action for each bot
        let mut actions: Vec<Action> = Vec::populate(&self.bots);

        // Populate default actions for all bots
        // TODO: Do we need to noop dead bots? Seems like the server will just ignore.
        let mut actions: Vec<Action> = Vec::populate(&self);
        let mut alive_bots = self.bots.iter().filter(|b| b.alive).map(|b| b.clone()).collect::<Vec<Bot>>();
        let mut no_alive_bots = alive_bots.len() as i16;

        // Try getting history events
        let historic_echoes = self.history.get( Event::Echo(defs::EchoEvent{pos: Pos{x:0,y:0}}), 5 );
        println!("Historic echo events {:?}", historic_echoes);

        for event in events {
            match *event {
                Damaged(ref ev) => {
                    println!("Evading on bot {}", ev.bot_id);
                    actions.push(self.evade_action(self.get_bot(ev.bot_id).unwrap()));
                }
                Detected(ref ev) => {
                    println!("Evading on bot {}", ev.bot_id);
                    actions.push(self.evade_action(self.get_bot(ev.bot_id).unwrap()));
                }
                Echo(ref ev) => {
                    // Filter alive bots
                    let mut radared = false;
                    for bot in &self.bots {
                        if bot.alive == true {
                            if !radared && no_alive_bots > 1 {
                                actions.set_action_for(bot.id, RADAR, ev.pos);
                                radared = true;
                            } else {
                                actions.set_action_for(bot.id, CANNON, ev.pos.random_spread());
                            }
                        }
                    }
                    println!("Got echo, gonna shoot at it!");
                    // actions.append(&mut self.all_shoot_at_action(&ev.pos));
                    println!("Actions from echo {:?}", actions);
                }
                See(ref ev) => {
                    println!("Saw something, gonna shoot at it!");
                    // actions.append(&mut self.all_shoot_at_action(&ev.pos));
                }
                _ => {}
            }
        }

        println!("Action is {:?}", actions);
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

    fn evade_pos(&self, bot: &Bot) -> Pos {
        let neighbours = bot.pos.neighbours(&self.config.moves_allowed);
        *rand::thread_rng().choose(&neighbours).expect("Oh there were no neighbors? That's impossible.")
    }

    fn all_shoot_at_action(&self, target: &Pos) -> Vec<Action> {
        return self.bots
            .iter()
            // TODO: Maybe add shuffle triangle here?
            // TODO: Random shooting at middle
            .zip(Pos::triangle_smart(target).iter())
            .map(|(bot, pos)| Action {
                bot_id: bot.id,
                action_type: CANNON.to_string(),
                pos: *pos,
            }).collect();
    }

    fn shoot_and_track_action(&self, target: &Pos) -> Vec<Action> {
        return self.bots
            .iter()
            .zip(Pos::triangle_down(target).iter())
            .map(|(bot, pos)| Action {
                bot_id: bot.id,
                action_type: if pos.x == 100 { RADAR.to_string() } else { CANNON.to_string() },
                pos: if pos.x == 100 { target.clone() } else { *pos },
            }).collect();
    }

    fn random_radars_action(&self) -> Vec<Action> {
        return self.bots.iter().map(|bot| Action {
            bot_id: bot.id,
            action_type: RADAR.to_string(),
            pos: util::get_random_pos(&self.radar_positions)
        }).collect();
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
                    enemy_positions.push(( None, ev.pos.clone() ));
                }
                Echo(ref ev) => {
                    enemy_positions.push(( None, ev.pos.clone() ));
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
                        self.history.add(&self.round_id, &events);
                        println!("Loggin events {:?}", &events);
                        println!("Logging History {:?}", &self.history);
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

trait HistoryList {
    fn add(&mut self, round_id: &i16, events: &Vec<Event>);
    fn filter_relevant(&self, events: &Vec<Event>) -> Vec<Event>;
    fn get(&self, match_event: Event, since: i16) -> Vec<(Event, i16)>;
}

impl HistoryList for Vec<HistoryEntry> {
    fn add(&mut self, round_id: &i16, events: &Vec<Event>) {
        let filtered_events = self.filter_relevant(events);
        self.push(HistoryEntry {
            round_id: *round_id,
            events: filtered_events
        });
    }

    fn filter_relevant(&self, events: &Vec<Event>) -> Vec<Event> {
        events
            .iter()
            .cloned()
            .filter(|e| {match *e {
                    Noaction(ref ev) => false,
                    Invalid => false,
                    _ => true,
                }})
            .collect()
    }

    // Returns each matching event as a tuple with round_id as first value
    fn get(&self, match_event: Event, since: i16) -> Vec<(Event, i16)> {
        let last_round = self.len() as i16 - 1;
        let historic_events = self
            .iter()
            .filter(|he| he.round_id > last_round - since  )
            .flat_map(|he| {
                let mut round_ids: Vec<i16> = Vec::new();
                for i in 0..he.events.len() {
                    round_ids.push(he.round_id);
                }
                he.events
                .iter()
                .cloned()
                .zip(round_ids)
                .filter(|e| {match e.0 {
                        Hit(ref ev) => {
                            match match_event {
                                Hit(ref ev) => true,
                                _ => false
                            }
                        },
                        Die(ref ev) => {
                            match match_event {
                                Die(ref ev) => true,
                                _ => false
                            }
                        },
                        See(ref ev) => {
                            match match_event {
                                See(ref ev) => true,
                                _ => false
                            }
                        },
                        Echo(ref ev) => {
                            match match_event {
                                Echo(ref ev) => true,
                                _ => false
                            }
                        },
                        Detected(ref ev) => {
                            match match_event {
                                Detected(ref ev) => true,
                                _ => false
                            }
                        },
                        Damaged(ref ev) => {
                            match match_event {
                                Damaged(ref ev) => true,
                                _ => false
                            }
                        },
                        Move(ref ev) => {
                            match match_event {
                                Move(ref ev) => true,
                                _ => false
                            }
                        },
                        _ => false,
                    }})
                // .map(|e| (&he.round_id as i16, e))
                }
            )
            .collect();
        return historic_events;
    }


}

trait ActionsList {
    // Naming?
    fn populate(ai: &Ai) -> Vec<Action>;
    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action>;
    fn set_action_for(&mut self, id: i16, action: &str, pos: Pos);
}

impl ActionsList for Vec<Action> {
    // Populate a default action for each bot with random radar
    fn populate(ai: &Ai) -> Vec<Action> {
        ai.bots
            .iter()
            .map(|b| Action {
                bot_id: b.id,
                action_type: RADAR.to_string(),
                pos: util::get_random_pos(&ai.radar_positions)
            })
            .collect::<Vec<Action>>()
    }

    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action> {
        self
            .iter_mut()
            .find(|ac|ac.bot_id == id)
    }

    fn set_action_for(&mut self, id: i16, action_type: &str, pos: Pos) {
        if let Some(action) = self.get_action_mut(id) {
            action.action_type = action_type.to_string();
            action.pos = pos;
        }
    }
}

#[derive(Debug)]
pub struct HistoryEntry {
    round_id: i16,
    events: Vec<Event>
}
