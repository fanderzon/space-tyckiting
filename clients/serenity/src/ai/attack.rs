use defs::{ Action, Event };
use strings::{ CANNON, RADAR, MOVE, SEE, RADARECHO, DIE };
use position::Pos;
use rand;
use rand::Rng;
use ai::*;
use lists::*;


impl Ai {
    pub fn attack_and_scan_if_target(&self, mut actions: &mut Vec<Action>) {
        // Let's attack if we've seen something in the last two turns
        // (if it was longer ago leave it to the scan task)
        let mut attack_events = self.history.get_events( SEE, 2 );
        attack_events.append(&mut self.history.get_events( RADARECHO, 2 ));

        println!("attack_and_scan_if_target, attack events events: {:?}", attack_events);

        for ev in attack_events {
            match ev.0 {
                Event::See(ref ev) => self.all_shoot_or_scan(&mut actions, ev.pos),
                Event::Echo(ref ev) => self.all_shoot_or_scan(&mut actions, ev.pos),
                _ => ()
            }
        }
    }

    // Takes the action array and a position to attack and modifies it
    pub fn all_shoot_or_scan(&self, actions: &mut Vec<Action>, target: Pos) {
        let die_events = self.history.get_events( DIE, 1 );
        if die_events.len() <= 0 {
            println!("All shoot or scan");
            let mut radared: bool = false;
            for bot in &self.bots {
                if bot.alive == true {
                    if Some(actions.get_action(bot.id)).unwrap().unwrap().action_type == MOVE.to_string() {
                        // Already on the move, let's keep it that way
                        println!("Already on the move, not changing bot {:?}", bot.id);
                    } else if radared == false && self.bots_alive() > 1 {
                        println!("Setting attack radar for bot {:?} {}", bot.id, radared);
                        actions.set_action_for(bot.id, RADAR, target);
                        radared = true;
                    } else {
                        actions.set_action_for(bot.id, CANNON, target.random_spread().clamp(&self.config.field_radius));
                        println!("Setting attack cannon for bot {:?}", bot.id);
                    }
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
