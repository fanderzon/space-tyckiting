use ai::*;
use util;
use defs:: { Action, Event, get_event_name };
use defs::Event::*;
use position::Pos;
use strings::{ RADAR, NOACTION, ALL, RADARECHO, SEE };


pub trait ActionsList {
    // Naming?
    fn populate(bots: &Vec<Bot>, radar_positions: &Vec<Pos>) -> Vec<Action>;
    fn get_action(&self, id: i16) -> Option<&Action>;
    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action>;
    fn set_action_for(&mut self, id: i16, action: &str, pos: Pos);
}

impl ActionsList for Vec<Action> {
    // Populate a default (radar) action for each bot with random radar
    fn populate(bots: &Vec<Bot>, radar_positions: &Vec<Pos>) -> Vec<Action> {
        bots
            .iter()
            .map(|b| Action {
                bot_id: b.id,
                action_type: RADAR.to_string(),
                pos: util::get_random_pos(radar_positions)
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
    fn add_events(&mut self, round_id: &i16, events: &Vec<Event>);
    fn add_actions(&mut self, round_id: &i16, actions: &Vec<Action>);
    fn get(&mut self, round_id: &i16) -> Option<&mut HistoryEntry>;
    fn filter_relevant(&self, events: &Vec<Event>) -> Vec<Event>;
    fn get_events(&self, match_event: &str, since: i16) -> Vec<(Event, i16)>;
    fn get_events_for_round(&self, match_event: &str, round_id: i16) -> Vec<Event>;
    fn get_last_enemy_position(&self) -> Option<(Event, i16)>;
    fn get_actions(&self, match_action: &str, since: i16) -> Vec<(Action, i16)>;
    fn get_actions_for_round(&self, match_action: &str, round_id: i16) -> Vec<Action>;
    fn get_action_for_bot(&self, bot_id: &i16, round_id: &i16) -> Option<Action>;
    fn set_mode(&mut self, mode: &str);
    fn get_mode(&self, round_id: &i16) -> String;
}

impl HistoryList for Vec<HistoryEntry> {
    fn add_events(&mut self, round_id: &i16, events: &Vec<Event>) {
        let filtered_events = self.filter_relevant(events);
        let mut new_entry: Option<HistoryEntry> = None;
        match self.get(&round_id) {
            Some(history_entry) => history_entry.events = filtered_events,
            None => {
                new_entry = Some(HistoryEntry {
                    round_id: *round_id,
                    events: filtered_events,
                    actions: Vec::new(),
                    mode: NOACTION.to_string(),
                });
            }
        }
        match new_entry {
            Some(history_entry) => self.push(history_entry),
            None => ()
        }
    }

    fn add_actions(&mut self, round_id: &i16, actions: &Vec<Action>) {
        let mut new_entry: Option<HistoryEntry> = None;
        let a = actions.iter().cloned().collect();
        match self.get(&round_id) {
            Some(history_entry) => history_entry.actions = a,
            None => {
                new_entry = Some(HistoryEntry {
                    round_id: *round_id,
                    events: Vec::new(),
                    actions: a,
                    mode: NOACTION.to_string(),
                });
            }
        }
        match new_entry {
            Some(history_entry) => self.push(history_entry),
            None => ()
        }
    }

    fn get(&mut self, round_id: &i16) -> Option<&mut HistoryEntry> {
        self
            .iter_mut()
            .find(|he|he.round_id == *round_id)
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

    // Returns each matching event as a tuple with round_id as second value
    fn get_events(&self, match_event: &str, since: i16) -> Vec<(Event, i16)> {
        let last_round = self.len() as i16 - 1;
        self
            .iter()
            .filter(|he| he.round_id > last_round - since  )
            .flat_map(|he| {
                // Slightly ugly work around for returning a tuple with the round_id
                let mut round_ids: Vec<i16> = Vec::new();
                for i in 0..he.events.len() {
                    round_ids.push(he.round_id);
                }
                he.events
                .iter()
                .cloned()
                .zip(round_ids)
                .filter(|e| get_event_name(&e.0) == match_event)
            })
            .collect()
    }

    fn get_events_for_round(&self, match_event: &str, round_id: i16) -> Vec<Event> {
        self
            .iter()
            .filter(|he| he.round_id == round_id)
            .flat_map(|he| he.events
                .iter()
                .cloned()
                .filter(|e| get_event_name(&e) == match_event))
            .collect()
    }

    fn get_last_enemy_position(&self) -> Option<(Event, i16)> {
        let last_entry = &self[self.len()-1];
        let mut round = last_entry.round_id + 0;
        while round > -1 {
            let mut see_events = self.get_events_for_round( RADARECHO, round );
            see_events.append(&mut self.get_events_for_round( SEE, round ));
            for event in see_events {
                return Some( (event, round) );
            }
            round -= 1;
        }
        return None;
    }

    // Returns each matching action as a tuple with round_id as second value
    fn get_actions(&self, match_action: &str, since: i16) -> Vec<(Action, i16)> {
        let last_round = self.len() as i16 - 1;
        self
            .iter()
            .filter(|he| he.round_id > last_round - since  )
            .flat_map(|he| {
                // Slightly ugly work around for returning a tuple with the round_id
                let mut round_ids: Vec<i16> = Vec::new();
                for i in 0..he.events.len() {
                    round_ids.push(he.round_id);
                }
                he.actions
                .iter()
                .cloned()
                .zip(round_ids)
                .filter(|e| e.0.action_type == match_action.to_string())
            })
            .collect()
    }

    fn get_actions_for_round(&self, match_action: &str, round_id: i16) -> Vec<Action> {
        self
            .iter()
            .filter(|he| he.round_id == round_id)
            .flat_map(|he| he.actions
                .iter()
                .cloned()
                .filter(|e| {
                    if match_action == ALL {
                        true
                    } else {
                        e.action_type == match_action.to_string()
                    }
                }))
            .collect()
    }



    fn get_action_for_bot(&self, bot_id: &i16, round_id: &i16) -> Option<Action> {
        self.get_actions_for_round( ALL, *round_id )
            .iter()
            .cloned()
            .find(|ac| ac.bot_id == *bot_id)
    }

    fn set_mode(&mut self, mode: &str) {
        let last = self.len() as i16 - 1;
        match self.get(&last) {
            Some(h) => h.mode = mode.to_string(),
            None => (),
        }
    }

    fn get_mode(&self, round_id: &i16) -> String {
        let last = self.len() - 1;
        self[last].mode.to_string()
    }
}

#[derive(Debug)]
pub struct HistoryEntry {
    pub round_id: i16,
    pub events: Vec<Event>,
    pub actions: Vec<Action>,
    pub mode: String,
}
