use std::collections::BTreeMap;
use defs::Action;
use position::Pos;
use strings::{RADAR, CANNON, NOACTION};
use ai::*;
use patterns::smart_scan_spread;
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


        let mut unexplored_echo = None;

        // Let's check if we were attacking last round
        if 0 != self.round_id && Attack == self.history.get_mode(self.round_id - 1) {
            // If so, and we switched back to scanning, let's see if we can find an echo we missed previously
            // Group the echo positions by round_id
            let mut echo_positions: Vec<(i16,Vec<Pos>)> = self.history.get_echo_positions(50)
                .iter()
                .filter(|tup| !self.is_pos_a_recorded_asteroid(&tup.0) )
                .fold(BTreeMap::new(), |mut acc, curr: &(Pos, i16)| {
                    acc.entry(curr.1).or_insert(vec![]).push(curr.0);
                    acc
                })
                .into_iter()
                .collect();

            echo_positions.retain(|tup| tup.1.len() > 1); // Only care about those with several notices.
            echo_positions.reverse(); // We want anti-chronological order.

            for (ref round_id, ref positions) in echo_positions {
                let cannon_actions = self.history.get_actions_for_round( CANNON, round_id + 1);
                let radar_actions = self.history.get_actions_for_round( RADAR, round_id + 1);

                // if there is a radar action it tells us exactly which position we went for
                let mut targeted_echo = None;
                if radar_actions.len() > 1 {
                    targeted_echo = Some(radar_actions[0].pos);
                } else if cannon_actions.len() > 0 {
                    let mut considered = cannon_actions[0].pos.neighbors(4);
                    considered.push(cannon_actions[0].pos);

                    let action_tup = considered
                        .iter()
                        .fold( (1000,Pos::new(0,0) ), |acc, pos| {
                            // Total distance of this position from all cannons positions
                            let d: i16 = cannon_actions
                                .iter()
                                .fold(0, |acc,curr| acc + curr.pos.distance(*pos));
                            // Accumulate the position with the lowest ditance
                            if d < acc.0 {
                                (d, *pos)
                            }  else {
                                acc
                            }
                        });
                    targeted_echo = Some(action_tup.1);
                }

                if let Some(targeted_echo) = targeted_echo {
                    if let Some(position) = positions.iter().find(|pos| pos.distance(targeted_echo) > 2) {
                        unexplored_echo = Some(*position);
                    }
                }
            }

        }

        // Do we have a lead of where to scan?
        if let Some(unexplored_echo) = unexplored_echo {
            self.logger.log(&format!("We picked up a previous echo at {}.", unexplored_echo), 2);
            idle_bots.iter()
                .zip(smart_scan_spread(unexplored_echo, idle_bots.len() as i16))
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
