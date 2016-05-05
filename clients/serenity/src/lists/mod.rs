use ai::*;
use util;
use defs::Action;
use position::Pos;
use strings::{ NOACTION };


pub trait ActionsList {
    // Naming?
    fn populate(bots: &Vec<Bot>) -> Vec<Action>;
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
