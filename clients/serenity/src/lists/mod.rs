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

pub trait BotList {
    fn render(&self) -> String;
}

impl BotList for Vec<Bot> {
    fn render(&self) -> String {
        if self.is_empty() {
            return String::from("¤ <no bots> ¤");
        } else {
            let mut result = String::from("¤");
            for bot in self {
                result.push_str(&format!(" {} ¤", bot));
            }
            return result;
        }
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
    fn add_events(&mut self, round_id: i16, events: &Vec<Event>);
    fn add_actions(&mut self, round_id: i16, actions: &Vec<Action>);
    fn get(&self, round_id: &i16) -> Option<&HistoryEntry>;
    fn get_mut(&mut self, round_id: i16) -> Option<&mut HistoryEntry>;
    fn filter_relevant(&self, events: &Vec<Event>) -> Vec<Event>;
    fn get_events(&self, match_event: &str, since: i16) -> Vec<(Event, i16)>;
    fn get_events_for_round(&self, match_event: &str, round_id: i16) -> Vec<Event>;
    fn get_last_enemy_position(&self) -> Option<(Event, i16)>;
    fn get_last_attack_action(&self) -> Option<(Action, i16)>;
    fn get_echo_positions(&self, since: i16) -> Vec<(Pos,i16)>;
    fn get_unused_echoes(&self, since: i16) -> Vec<(Pos,i16)>;
    fn get_actions(&self, match_action: &str, since: i16) -> Vec<(Action, i16)>;
    fn get_actions_for_round(&self, match_action: &str, round_id: i16) -> Vec<Action>;
    fn get_action_for_bot(&self, bot_id: &i16, round_id: &i16) -> Option<Action>;
    fn set_mode(&mut self, round_id: &i16, mode: ActionMode);
    fn get_mode(&self, round_id: i16) -> ActionMode;
    fn set_decision(&mut self, round_id: i16, mode: Decision);
    fn get_decision(&self, round_id: i16) -> Decision;
}

impl HistoryList for Vec<HistoryEntry> {
    #[allow(dead_code)]
    fn add_events(&mut self, round_id: i16, events: &Vec<Event>) {
        debug_assert!(0 <= round_id && round_id <= self.len() as i16, "Adding either to existing round or to nextcoming one.");
        let filtered_events = self.filter_relevant(events);
        if self.len() as i16 > round_id {
            if let Some(history_entry) = self.get_mut(round_id) {
                history_entry.events = filtered_events;
            }
        } else {
            self.push(HistoryEntry {
                round_id: round_id,
                events: filtered_events,
                actions: Vec::new(),
                decision: Decision::with_defaults(),
            });
        }
    }

    #[allow(dead_code)]
    fn add_actions(&mut self, round_id: i16, actions: &Vec<Action>) {
        debug_assert!(0 <= round_id && round_id <= self.len() as i16, "Adding either to existing round or to nextcoming one.");
        let a = actions.iter().cloned().collect();
        if self.len() as i16 > round_id {
            if let Some(history_entry) = self.get_mut(round_id) {
                history_entry.actions = a;
            }
        } else {
            self.push(HistoryEntry {
                round_id: round_id,
                events: Vec::new(),
                actions: a,
                decision: Decision::with_defaults(),
            });
        }
    }

    #[allow(dead_code)]
    fn get(&self, round_id: &i16) -> Option<&HistoryEntry> {
        debug_assert!(0 <= *round_id && *round_id < self.len() as i16);
        self.iter()
            .find(|he|he.round_id == *round_id)
    }

    #[allow(dead_code)]
    fn get_mut(&mut self, round_id: i16) -> Option<&mut HistoryEntry> {
        debug_assert!(0 <= round_id && round_id < self.len() as i16);
        self.iter_mut()
            .find(|he|he.round_id == round_id)
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
    // Pass 1 for since if you want the current round.
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
    // Returned in chronological order.
    #[allow(dead_code)]
    fn get_echo_positions(&self, since: i16) -> Vec<(Pos,i16)> {
        debug_assert!(since >= 0);
        // get all echo positions
        let mut see_events = self.get_events( SEE, since );
        see_events.append(&mut self.get_events( RADARECHO, since ));
        see_events.sort_by(|a, b| a.1.cmp(&b.1));

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

    // Returns unused echoes that has been logged to history.decision.unused_echoes
    // Positions that have been used as target in later rounds are filtered out
    // The vector is sorted in reverse round_id order
    fn get_unused_echoes(&self, since: i16) -> Vec<(Pos,i16)> {
        let last_round = self.len() as i16 - 1;
        let mut echoes: Vec<(Pos,i16)> = self
            .iter()
            .filter(|he| he.round_id > last_round - since)
            .flat_map(|he| {
                // Slightly ugly work around for returning a tuple with the round_id
                let mut round_ids: Vec<i16> = Vec::new();
                for _ in 0..he.decision.unused_echoes.len() {
                    round_ids.push(he.round_id);
                }
                he.decision.unused_echoes
                    .iter()
                    .cloned()
                    .zip(round_ids)
            })
            .filter(|&(pos, round_id)| {
                let count: usize = self
                    .iter()
                    .filter(|he| he.decision.target.is_some())
                    .filter(|he| {
                        // filter out "unused echoes" that has been used as a target in a later round
                        {he.decision.target.unwrap() == pos && he.round_id > round_id}
                    })
                    .count();
                count == 0
            })
            .collect();
        // Sort reverse by round_id
        echoes.sort_by(|a, b| b.1.cmp(&a.1));
        echoes
    }

    // Returns each matching action as a tuple with round_id as second value
    #[allow(dead_code,unused_variables)]
    fn get_actions(&self, match_action: &str, since: i16) -> Vec<(Action, i16)> {
        debug_assert!(since >= 0);
        let last_round = self.len() as i16 - 1;
        self.iter()
            .filter(|he| he.round_id > last_round - since)
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
        debug_assert!(0 <= round_id && round_id < self.len() as i16);
        self.iter()
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
        debug_assert!(0 <= *round_id && *round_id < self.len() as i16);
        debug_assert!(0 <= *bot_id);
        self.get_actions_for_round( ALL, *round_id )
            .iter()
            .cloned()
            .find(|ac| ac.bot_id == *bot_id)
    }

    #[allow(dead_code)]
    fn set_mode(&mut self, round_id: &i16, mode: ActionMode) {
        debug_assert!(0 <= *round_id && *round_id < self.len() as i16);
        match self.get_mut(*round_id) {
            Some(history_entry) => history_entry.decision.mode = mode,
            None => ()
        }
    }

    #[allow(dead_code)]
    fn get_mode(&self, round_id: i16) -> ActionMode {
        debug_assert!(0 <= round_id && round_id < self.len() as i16);
        self[round_id as usize].decision.mode.clone()
    }

    #[allow(dead_code)]
    fn set_decision(&mut self, round_id: i16, decision: Decision) {
        debug_assert!(0 <= round_id && round_id < self.len() as i16);
        match self.get_mut(round_id) {
            Some(history_entry) => history_entry.decision = decision,
            None => ()
        }
    }

    #[allow(dead_code)]
    fn get_decision(&self, round_id: i16) -> Decision {
        debug_assert!(0 <= round_id && round_id < self.len() as i16);
        self[round_id as usize].decision.clone()
    }
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub round_id: i16,
    pub events: Vec<Event>,
    pub actions: Vec<Action>,
    pub decision: Decision,
}

#[derive(Debug, Clone)]
pub struct Decision {
    // Attack or Scan
    pub mode: ActionMode,
    // The target Pos of the attack or scan (actual actions may have other positions because of spread)
    pub target: Option<Pos>,
    // Echoes we got this round, but did not act on
    pub unused_echoes: Vec<Pos>,
}

impl Decision {
    pub fn with_defaults() -> Decision {
        Decision {
            mode: ActionMode::Nomode,
            target: None,
            unused_echoes: Vec::new(),
        }
    }

    pub fn add_attack_decision(&mut self, target: &Pos, echoes: &Vec<Pos>) {
        self.mode = ActionMode::Attack;
        self.target = Some(*target);
        self.unused_echoes = echoes
            .into_iter()
            .filter(|echo| *echo != target)
            .cloned()
            .collect();
    }
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
