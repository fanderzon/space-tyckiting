use ai::*;

impl Ai {
    fn evade_action(&self, bot: &Bot) -> Action {
        if self.botsAlive() >= 2 {
            return self.evade_spread();
        } else {
            return self.evade_random();
        }
    }

    fn evade_random(&self, bot: &Bot) -> Action {
        let neighbors = bot.pos.neighbors(&self.config.moves_allowed); 
        let move_to = *rand::thread_rng().choose(&neighbors).expect("Oh there were no neighbors? That's impossible.");
        println!("MOVES: {}, {}, {}, {}", bot.pos.x, bot.pos.y, move_to.x, move_to.y);
        return Action {
            bot_id: bot.id,
            action_type: MOVE.to_string(),
            pos: move_to,
        };
    }

    fn evade_spread(&self, bot: &Bot) -> Action {
        let neighbors = bot.pos.neighbors(&self.config.moves_allowed); 
        let otherbots: Vec<&Bot> = self.bots.iter()
            .by_ref()
            .filter(|otherbot| otherbot.id != bot.id)
            .collect();
            
        let move_to: &Pos = neighbors.iter()
            .max_by_key( |pos| otherbots.iter()
                .map( |otherbot| pos.distance(otherbot.pos))
                .min()
                .expect("There should be other bots"))
            .expect("There should be neighbor positions");

        return Action {
            bot_id: bot.id,
            action_type: MOVE.to_string(),
            pos: move_to.clone(),
        };
    }
}
