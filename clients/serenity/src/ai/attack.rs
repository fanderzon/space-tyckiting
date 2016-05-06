use defs::Action;
use strings::{ CANNON, RADAR };
use position::Pos;
use rand;
use rand::Rng;
use ai::*;
use lists::*;


impl Ai {
    // Takes the action array and a position to attack and modifies it
    pub fn all_shoot_or_scan(&self, actions: &mut Vec<Action>, target: Pos) {
        let mut radared = false;
        for bot in &self.bots {
            if bot.alive == true {
                if !radared && self.bots_alive() > 1 {
                    actions.set_action_for(bot.id, RADAR, target);
                    radared = true;
                } else {
                    actions.set_action_for(bot.id, CANNON, target.random_spread(target));
                }
            }
        }
    }

    pub fn all_shoot_at_action(&self, actions: &mut Vec<Action>, target: &Pos) {
        self.bots
            .iter()
            // TODO: Maybe add shuffle triangle here?
            // TODO: Random shooting at middle
            .zip(Pos::triangle_smart(target).iter())
            .map(|(bot, pos)| {
                actions.set_action_for(bot.id, CANNON, *pos);
                Action {
                    bot_id: bot.id,
                    action_type: CANNON.to_string(),
                    pos: *pos,
            }}).count();
    }
}
