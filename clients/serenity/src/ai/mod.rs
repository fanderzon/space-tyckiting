extern crate serde; extern crate serde_json;

use std::str::from_utf8;
use websocket::Message;
use websocket::message::Type;
use position::Pos;
use util;
use defs;
use defs::{Event, Action, Config, ActionsMessage, IncomingMessage, IncomingEvents};
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
    // Snapshots of known enemy positions for every round, last being this one
    enemy_poss: Vec<Vec<(Option<i16>, Pos)>>,
    // Same as above, but opposite: What the enemy knows for sure about our positions
    enemy_knowledge: Vec<Vec<(i16, Pos)>>,
    damaged_bots: Vec<Vec<i16>>,
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

        // Add random radar actions as default
        self.random_radars_action(&mut actions);

        // let mut actions = self.random_radars_action();
        let curr_enemy_pos: &Vec<(Option<i16>, Pos)> = self.enemy_poss.last().expect("There should be an enemy pos snapshot for this round!");
        let curr_enemy_know   = self.enemy_knowledge.last().expect("There should be a snapshot for enemy knowledge");
        let curr_damaged_bots = self.damaged_bots.last().expect("There should be an damaged bots snapshot for this round!");

        // TODO: Handle multiple known positions better
        if let Some(tup) = curr_enemy_pos.first() {
            let (_, ref pos) = *tup;
            self.all_shoot_at_action(&mut actions, pos);
        }

        for tup in curr_enemy_know {
            let (ref id, _) = *tup;
            actions.set_action_for(*id, MOVE, self.evade_pos(self.get_bot(*id).unwrap()));
        }

        for id in curr_damaged_bots {
            actions.set_action_for(*id, MOVE, self.evade_pos(self.get_bot(*id).unwrap()));
        }

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
            enemy_poss: Vec::new(),
            // Same as above, but opposite: What the enemy knows for sure about our positions
            enemy_knowledge: Vec::new(),
            damaged_bots: Vec::new(),
            config: start.config.clone(),
        };
    }

    fn bots_alive(&self) -> usize {
        self.bots.iter().filter(|bot| bot.alive ).count()
    }

    fn all_shoot_at_action(&self, actions: &mut Vec<Action>, target: &Pos) {
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

    fn random_radars_action(&self, actions: &mut Vec<Action>) {
        for bot in &self.bots {
            if bot.alive {
                actions.set_action_for(bot.id, RADAR, util::get_random_pos(&self.radar_positions));
            }
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
        let mut enemy_positions = Vec::new();
        let mut enemy_knowledge = Vec::new();
        let mut damaged_bots = Vec::new();

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
                    damaged_bots.push(ev.bot_id);
                }
                Move(ref ev) => {
                    let mut bot = self.get_bot_mut(ev.bot_id).expect("No bot on our team with this id wtf?");
                    bot.pos = ev.pos;
                }
                Detected(ref ev) => {
                    enemy_knowledge.push( (ev.bot_id, self.get_bot(ev.bot_id).expect("Not bot on our team with this id").pos.clone()) );
                }
                Noaction(_) => {
                    //TODO: Maybe we can use the knowledge that a bot is sleeping? To exploit bugs
                    //in enemy code ;)
                }
                _ => {}
            }
        }

        // TODO: Maybe remove dupes? There are edge cases...
        self.enemy_poss.push(enemy_positions);
        self.enemy_knowledge.push(enemy_knowledge);
        self.damaged_bots.push(damaged_bots);
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
                        let decisions = self.make_decisions();
                        let actions_message = self.make_actions_message(decisions);
                        return Ok(actions_message);
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
}
