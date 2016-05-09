use defs::{ Action, Event };
use strings::{DETECTED, DAMAGED, MOVE};
use position::Pos;
use rand;
use rand::Rng;
use ai::*;
use lists::*;
use ai::bot::Bot;

impl Ai {
    #[allow(dead_code)]
    pub fn evade_action(&self, bot: &Bot) -> Action {
        if self.bots_alive() >= 2 {
            return self.evade_spread(bot);
        } else {
            return self.evade_random(bot);
        }
    }

    pub fn evade_pos(&self, bot: &Bot) -> Pos {
        if self.bots_alive() >= 2 {
            return self.evade_spread_pos(bot);
        } else {
            return self.evade_random_pos(bot);
        }
    }

    pub fn evade_if_needed(&mut self, actions: &mut Vec<Action>) {
        // Let's evade if we were seen in the last two turns
        let mut evade_events = self.history.get_events( DETECTED, 2 );
        evade_events.append(&mut self.history.get_events( DAMAGED, 2 ));

        for ev in evade_events {
            match ev.0 {
                Event::Detected(ref ev) => {
                    let destination = self.evade_pos(self.get_bot(ev.bot_id).unwrap());
                    actions.set_action_for(ev.bot_id, MOVE, destination);
                    self.logger.log(&format!("Evading Bot {} to {}", ev.bot_id, destination), 2);
                }
                Event::Damaged(ref ev) => {
                    let destination = self.evade_pos(self.get_bot(ev.bot_id).unwrap());
                    actions.set_action_for(ev.bot_id, MOVE, destination);
                    self.logger.log(&format!("Evading Bot {} to {}", ev.bot_id, destination), 2);
                }
                _ => ()
            }
        }
    }

    #[allow(dead_code)]
    fn evade_random(&self, bot: &Bot) -> Action {
        let move_to = self.evade_random_pos(&bot);
        println!("MOVES: {}, {}, {}, {}", bot.pos.x, bot.pos.y, move_to.x, move_to.y);
        return Action {
            bot_id: bot.id,
            action_type: MOVE.to_string(),
            pos: move_to,
        };
    }

    fn evade_random_pos(&self, bot: &Bot) -> Pos {
        let neighbors = bot.pos.clamped_neighbors(&self.config.moves_allowed, &self.config.field_radius);
        *rand::thread_rng()
            .choose(&neighbors)
            .expect("Oh there were no neighbors? That's impossible.")
    }

    #[allow(dead_code)]
    fn evade_spread(&self, bot: &Bot) -> Action {
        let move_to = self.evade_spread_pos(&bot);

        return Action {
            bot_id: bot.id,
            action_type: MOVE.to_string(),
            pos: move_to.clone(),
        };
    }

    fn evade_spread_pos(&self, bot: &Bot) -> Pos {
        // When too close to the edge of the board the spread logic doesn't move enough
        if bot.pos.distance(Pos{x: 0, y: 0}) > &self.config.field_radius - 2 {
            return self.evade_random_pos(&bot);
        }
        let neighbors = bot.pos.clamped_neighbors(&self.config.moves_allowed, &self.config.field_radius);
        let otherbots: Vec<&Bot> = self.bots.iter()
            .by_ref()
            .filter(|otherbot| otherbot.id != bot.id)
            .collect();

        neighbors.iter()
            .max_by_key( |pos| otherbots.iter()
                .map( |otherbot| pos.distance(otherbot.pos))
                .min()
                .expect("There should be other bots"))
            .expect("There should be neighbor positions")
            .clone()
    }
}
