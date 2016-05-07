use ai::*;
use util;
use defs:: { Action, Event, get_event_name };
use defs::Event::*;
use position::Pos;
use strings::{ RADAR };


pub trait ActionsList {
    // Naming?
    fn populate(bots: &Vec<Bot>, radar_positions: &Vec<Pos>) -> Vec<Action>;
    fn get_action(&self, id: i16) -> Option<&Action>;
    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action>;
    fn set_action_for(&mut self, id: i16, action: &str, pos: Pos);
}

impl ActionsList for Vec<Action> {
    // Populate a default action for each bot with random radar
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

    // Returns each matching event as a tuple with round_id as first value
    fn get_events(&self, match_event: &str, since: i16) -> Vec<(Event, i16)> {
        let last_round = self.len() as i16 - 1;
        let historic_events = self
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
            .collect();
        return historic_events;
    }
}

#[derive(Debug)]
pub struct HistoryEntry {
    pub round_id: i16,
    pub events: Vec<Event>,
    pub actions: Vec<Action>
}
