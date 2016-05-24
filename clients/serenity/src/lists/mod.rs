use std::fmt;
use defs:: { Action, Event, get_event_name };
use defs::Event::*;
use position::Pos;
use strings::{ NOACTION, ALL, RADARECHO, SEE, CANNON };
use ai::bot::Bot;

pub trait AsteroidList {
    fn register(&mut self, pos: Pos);
    fn register_maybe(&mut self, pos: Pos);
    fn is_asteroid(&self, pos: Pos) -> bool;
    fn is_maybe_asteroid(&self, pos: Pos) -> bool;
}

impl AsteroidList for Vec<(Pos, bool)> {
    fn register(&mut self, pos: Pos) {
        if !self.is_asteroid(pos) {
            self.retain(|tup|tup.0 != pos);
            self.push((pos, true));
        }
    }

    fn register_maybe(&mut self, pos: Pos) {
        if !self.is_maybe_asteroid(pos) {
            self.push((pos, false));
        }
    }

    fn is_asteroid(&self, pos: Pos) -> bool {
        self.iter()
            .find(|tup| tup.0 == pos && tup.1)
            .is_some()
    }

    fn is_maybe_asteroid(&self, pos: Pos) -> bool {
        self.iter()
            .find(|tup| tup.0 == pos)
            .is_some()
    }
}

pub trait ActionsList {
    // Naming?
    fn populate(bots: &Vec<Bot>) -> Vec<Action>;
    fn get_action(&self, id: i16) -> Option<&Action>;
    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action>;
    fn set_action_for(&mut self, id: i16, action: &str, pos: Pos);
    fn render(&self) -> String;
}

impl ActionsList for Vec<Action> {
    // Populate a default (radar) action for each bot with random radar
    #[allow(dead_code)]
    fn populate(bots: &Vec<Bot>) -> Vec<Action> {
        bots.iter()
            .map(|b| Action {
                bot_id: b.id,
                action_type: NOACTION.to_string(),
                pos: Pos {x: 0, y: 0},
            })
            .collect::<Vec<Action>>()
    }

    #[allow(dead_code)]
    fn get_action(&self, id: i16) -> Option<&Action> {
        self.iter()
            .find(|ac|ac.bot_id == id)
    }

    #[allow(dead_code)]
    fn get_action_mut(&mut self, id: i16) -> Option<&mut Action> {
        self.iter_mut()
            .find(|ac|ac.bot_id == id)
    }

    #[allow(dead_code)]
    fn set_action_for(&mut self, id: i16, action_type: &str, pos: Pos) {
        let opt_act = self.get_action_mut(id);
        debug_assert!(opt_act.is_some());
        if let Some(action) = opt_act {
            action.action_type = action_type.to_string();
            action.pos = pos;
        }
    }

    // Rust y u no let me make this as a trait???
    fn render(&self) -> String {
        if self.is_empty() {
            return String::from("| <no actions> |");
        } else {
            let mut result = String::from("|");
            for ac in self {
                result.push_str(&format!(" {} |", ac));
            }
            return result;
        }
    }
}

pub trait HistoryList {
    fn add_events(&mut self, round_id: &i16, events: &Vec<Event>);
    fn add_actions(&mut self, round_id: &i16, actions: &Vec<Action>);
    fn get(&self, round_id: &i16) -> Option<&HistoryEntry>;
    fn get_mut(&mut self, round_id: &i16) -> Option<&mut HistoryEntry>;
    fn filter_relevant(&self, events: &Vec<Event>) -> Vec<Event>;
    fn get_events(&self, match_event: &str, since: i16) -> Vec<(Event, i16)>;
    fn get_events_for_round(&self, match_event: &str, round_id: i16) -> Vec<Event>;
    fn get_last_enemy_position(&self) -> Option<(Event, i16)>;
    fn get_last_attack_action(&self) -> Option<(Action, i16)>;
    fn get_echo_positions(&self, since: i16) -> Vec<(Pos,i16)>;
    fn get_actions(&self, match_action: &str, since: i16) -> Vec<(Action, i16)>;
    fn get_actions_for_round(&self, match_action: &str, round_id: i16) -> Vec<Action>;
    fn get_action_for_bot(&self, bot_id: &i16, round_id: &i16) -> Option<Action>;
    fn set_mode(&mut self, round_id: &i16, mode: ActionMode);
    fn get_mode(&self, round_id: &i16) -> ActionMode;
}

impl HistoryList for Vec<HistoryEntry> {
    #[allow(dead_code)]
    fn add_events(&mut self, round_id: &i16, events: &Vec<Event>) {
        debug_assert!(0 <= *round_id && *round_id <= self.len() as i16, "Adding either to existing round or to nextcoming one.");
        let filtered_events = self.filter_relevant(events);
        let mut new_entry: Option<HistoryEntry> = None;
        match self.get_mut(&round_id) {
            Some(history_entry) => history_entry.events = filtered_events,
            None => {
                new_entry = Some(HistoryEntry {
                    round_id: *round_id,
                    events: filtered_events,
                    actions: Vec::new(),
                    mode: ActionMode::Nomode,
                });
            }
        }
        match new_entry {
            Some(history_entry) => self.push(history_entry),
            None => ()
        }
    }

    #[allow(dead_code)]
    fn add_actions(&mut self, round_id: &i16, actions: &Vec<Action>) {
        debug_assert!(0 <= *round_id && *round_id <= self.len() as i16, "Adding either to existing round or to nextcoming one.");
        let mut new_entry: Option<HistoryEntry> = None;
        let a = actions.iter().cloned().collect();
        match self.get_mut(&round_id) {
            Some(history_entry) => history_entry.actions = a,
            None => {
                new_entry = Some(HistoryEntry {
                    round_id: *round_id,
                    events: Vec::new(),
                    actions: a,
                    mode: ActionMode::Nomode,
                });
            }
        }
        match new_entry {
            Some(history_entry) => self.push(history_entry),
            None => ()
        }
    }

    #[allow(dead_code)]
    fn get(&self, round_id: &i16) -> Option<&HistoryEntry> {
        debug_assert!(0 <= *round_id && *round_id < self.len() as i16);
        self.iter()
            .find(|he|he.round_id == *round_id)
    }

    #[allow(dead_code)]
    fn get_mut(&mut self, round_id: &i16) -> Option<&mut HistoryEntry> {
        debug_assert!(0 <= *round_id && *round_id < self.len() as i16);
        self.iter_mut()
            .find(|he|he.round_id == *round_id)
    }

    #[allow(dead_code)]
    fn filter_relevant(&self, events: &Vec<Event>) -> Vec<Event> {
        events.iter()
            .cloned()
            .filter(|e| {match *e {
                    Noaction(_) => false,
                    Invalid => false,
                    _ => true,
                }})
            .collect()
    }

    // Returns each matching event as a tuple with round_id as second value
    #[allow(dead_code,unused_variables)]
    fn get_events(&self, match_event: &str, since: i16) -> Vec<(Event, i16)> {
        debug_assert!(since >= 0);
        let last_round = self.len() as i16 - 1;
        self.iter()
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

    #[allow(dead_code)]
    fn get_events_for_round(&self, match_event: &str, round_id: i16) -> Vec<Event> {
        debug_assert!(0 <= round_id && round_id < self.len() as i16);
        self.iter()
            .filter(|he| he.round_id == round_id)
            .flat_map(|he| he.events
                .iter()
                .cloned()
                .filter(|e| get_event_name(&e) == match_event))
            .collect()
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn get_last_attack_action(&self) -> Option<(Action, i16)> {
        let last_entry = &self[self.len()-1];
        let mut round = last_entry.round_id + 0;
        while round > -1 {
            let cannon_actions = self.get_actions_for_round( CANNON, round );
            for action in cannon_actions {
                return Some( (action, round) );
            }
            round -= 1;
        }
        return None;
    }

    // Convenience method returning an optional tuple of Pos and round_id for all see/echo events
    #[allow(dead_code)]
    fn get_echo_positions(&self, since: i16) -> Vec<(Pos,i16)> {
        debug_assert!(since >= 0);
        // get all echo positions
        let mut see_events = self.get_events( SEE, since );
        see_events.append(&mut self.get_events( RADARECHO, since ));

        see_events
            .iter()
            .cloned()
            .map(|tup| {
                match tup.0 {
                    Event::See(ref ev) => (ev.pos.clone(), tup.1),
                    Event::Echo(ref ev) => (ev.pos.clone(), tup.1),
                    _ => (Pos::origo(), 0)
                }
            })
            .collect()
    }

    // Returns each matching action as a tuple with round_id as second value
    #[allow(dead_code,unused_variables)]
    fn get_actions(&self, match_action: &str, since: i16) -> Vec<(Action, i16)> {
        debug_assert!(since >= 0);
        let last_round = self.len() as i16 - 1;
        self.iter()
            .filter(|he| he.round_id > last_round - since  )
            .flat_map(|he| {
                // Slightly ugly work around for returning a tuple with the round_id
                let mut round_ids: Vec<i16> = Vec::new();
                for i in 0..he.events.len() {
                    round_ids.push(he.round_id);
                }
                he.actions.iter()
                    .cloned()
                    .zip(round_ids)
                    .filter(|e| e.0.action_type == match_action.to_string())
            })
            .collect()
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    fn get_action_for_bot(&self, bot_id: &i16, round_id: &i16) -> Option<Action> {
        self.get_actions_for_round( ALL, *round_id )
            .iter()
            .cloned()
            .find(|ac| ac.bot_id == *bot_id)
    }

    #[allow(dead_code)]
    fn set_mode(&mut self, round_id: &i16, mode: ActionMode) {
        match self.get_mut(&round_id) {
            Some(history_entry) => history_entry.mode = mode,
            None => ()
        }
    }

    #[allow(dead_code)]
    fn get_mode(&self, round_id: &i16) -> ActionMode {
        self[*round_id as usize].mode.clone()
    }
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub round_id: i16,
    pub events: Vec<Event>,
    pub actions: Vec<Action>,
    pub mode: ActionMode,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActionMode {
    Attack,
    Scan,
    Nomode,
}

impl fmt::Display for ActionMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let as_str = match self {
            &ActionMode::Attack => "ATTACK",
            &ActionMode::Scan => "SCAN",
            &ActionMode::Nomode => "NOMODE",
        };
        write!(f, "{}", as_str)
    }
}

