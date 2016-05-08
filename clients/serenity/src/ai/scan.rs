use defs::Action;
use strings::{RADAR};
use ai::*;
use util;
use lists::ActionsList;

impl Ai {
    pub fn random_radars_action(&self, actions: &mut Vec<Action>) {
        for bot in &self.bots {
            if bot.alive {
                actions.set_action_for(bot.id, RADAR, util::get_random_pos(&self.radar_positions.1));
            }
        }
    }

    pub fn scan_with_idle_bots(&mut self, actions: &mut Vec<Action>) {
        let (ref mut radar_index, ref positions) = self.radar_positions;
        let idle_bots: &Vec<i16> = &actions.iter()
            .cloned()
            .filter(|ac| ac.action_type == RADAR.to_string())
            .map(|ref ac| ac.bot_id)
            .collect::<Vec<i16>>();

        for bot_id in idle_bots {
            // replace with better radar logic
            if *radar_index > positions.len() as i16 - 1 {
                *radar_index = 0;
            }
            println!("Scanning with radar index {:?} {}", radar_index, positions.len());
            actions.set_action_for(*bot_id, RADAR, self.radar_positions.1[*radar_index as usize]);
            *radar_index += 1;
        }
    }
}
