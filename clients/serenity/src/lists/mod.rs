use ai::*;
use util;
use defs:: { Action, Event };
use defs::Event::*;
use position::Pos;
use strings::{ NOACTION };


pub trait ActionsList {
    // Naming?
    fn populate(bots: &Vec<Bot>) -> Vec<Action>;
    fn get_action(&self, id: i16) -> Option<&Action>;
    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action>;
    fn set_action_for(&mut self, id: i16, action: &str, pos: Pos);
}

impl ActionsList for Vec<Action> {
    // Populate a default action for each bot with random radar
    fn populate(bots: &Vec<Bot>) -> Vec<Action> {
        bots
            .iter()
            .map(|b| Action {
                bot_id: b.id,
                action_type: NOACTION.to_string(),
                pos: Pos { x: 0, y: 0 }
            })
            .collect::<Vec<Action>>()
    }

    fn get_action(&self, id: i16) -> Option<&Action> {
        self
            .iter()
            .find(|ac|ac.bot_id == id)
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

pub trait HistoryList {
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

#[derive(Debug)]
pub struct HistoryEntry {
    pub round_id: i16,
    pub events: Vec<Event>
}
