use defs::Action;
use position::Pos;
use strings::{RADAR, NOACTION};
use ai::*;
use util;
use lists::{ ActionsList, HistoryList };
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

    pub fn scan_with_idle_bots(&mut self, actions: &mut Vec<Action>) {
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


        // Let's check if we were attacking last round
        match self.history.get_mode(self.round_id - 1) {
            // If so, and we switched back to scanning, let's see if we can find an echo we missed previously
            Attack => {

                let echo_positions: Vec<(i16,Vec<Pos>)> = self.history.get_echo_positions(30)
                    .iter()
                    .fold(vec![], |mut acc, curr| {
                        let &(pos, round_id) = curr;
                        if acc.iter().find(|tup| tup.0 == round_id).is_some() {
                            if let Some(tup) = acc.iter_mut().find(|tup| tup.0 == round_id) {
                                tup.1.push(pos);
                            }
                        } else {
                            acc.push( (round_id, vec![pos]) );
                        }
                        acc
                    });


                for (ref round_id, ref positions) in echo_positions.iter().rev().cloned().collect::<Vec<_>>() {
                    println!("round_id: {}: {:?}", round_id, positions);

                }
            },
            _ => {
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
            },
        }
    }
}
