use defs::Action;
use strings::{RADAR};
use ai::*;
use util;
use lists::ActionsList;

impl Ai {
    pub fn random_radars_action(&self, actions: &mut Vec<Action>) {
        for bot in &self.bots {
            if bot.alive {
                actions.set_action_for(bot.id, RADAR, util::get_random_pos(&self.radar_positions));
            }
        }
    }
}
