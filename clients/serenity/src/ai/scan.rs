use defs::Action;
use strings::{RADAR, NOACTION};
use ai::*;
use patterns::smart_scan_spread;
use util;
use lists::{ ActionsList, HistoryList, Decision };
use lists::ActionMode::*;

impl Ai {
    // Might be good to keep around as an option?
    #[allow(dead_code)]
    pub fn random_radars_action(&self, actions: &mut Vec<Action>) {
        for bot in &self.bots {
            if bot.alive {
                actions.set_action_for(bot.id, RADAR, util::get_random_pos(&self.radar_positions.1));
            }
        }
    }

    pub fn scan_with_idle_bots(&mut self, actions: &mut Vec<Action>, decision: &mut Decision) {
        // Get the bots available for scanning
        let idle_bots: Vec<i16> = self.bots
            .iter()
            .cloned()
            .filter(|bot| bot.alive && {
                if let Some(ac) = actions.iter().find(|ac| ac.bot_id == bot.id) {
                    ac.action_type == NOACTION.to_string()
                } else {
                    false
                }
            })
            .map(|ref bot| bot.id)
            .collect();

        decision.mode = Scan;

        let unused_echoes = self.history.get_unused_echoes(50);
        if unused_echoes.len() > 0 {
            let (unused_pos, unused_round_id) = unused_echoes[0];
            println!("We picked up a previous echo at {} from round {}.", unused_pos, unused_round_id);
            self.logger.log(&format!("We picked up a previous echo at {} from round {}.", unused_pos, unused_round_id), 2);
            idle_bots
                .iter()
                .zip(smart_scan_spread(unused_pos, idle_bots.len() as i16))
                .map(|(&ref bot_id, ref pos)| {
                    self.logger.log(&format!("Scanning with Bot {} on {} b/c it was idle and we picked up a historic echo.", bot_id, pos), 2);
                    actions.set_action_for(*bot_id, RADAR, pos.clamp(&self.config.field_radius))
                })
                .count();
        } else {
            // Resume basic sequential scanning
            let (ref mut radar_index, ref positions) = self.radar_positions;
            for bot_id in idle_bots {
                // replace with better radar logic
                if *radar_index > positions.len() as i16 - 1 {
                    *radar_index = 0;
                }
                let target = self.radar_positions.1[*radar_index as usize];
                actions.set_action_for(bot_id, RADAR, target);
                *radar_index += 1;

                self.logger.log(&format!("Scanning with Bot {} on {} b/c it was idle.", bot_id, target), 2);
            }
        }
    }
}
