/** Rust Tyckiting client - A websocket client for a fight to kill all other bots
 *  Copyright Futurice Oy (2015)
 *
 *  This file is part of Rust Tyckiting client.
 *
 *  Rust Tyckiting client is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Rust Tyckiting client is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Rust Tyckiting client.  If not, see <http://www.gnu.org/licenses/>.
 */
extern crate rand;

use super::incoming::{Event, Team, TeamNoPosNoHp};
use super::{Position, GameConfig};

use self::rand::{thread_rng, Rng};
use std::default::Default;

pub trait Ai {
    fn respond(&mut self, Vec<Event>) -> Vec<Action>;
    fn set_state(&mut self, config: GameConfig, you: Team, other_teamss: Vec<TeamNoPosNoHp>) -> ();
}

#[derive(Default)]
struct RandomAi {
    config: GameConfig,
    you: Team,
    other_teams: Vec<TeamNoPosNoHp>
}

impl Ai for RandomAi {
    #[allow(unused_variables)]
    fn respond(&mut self, events: Vec<Event>) -> Vec<Action>  {
        self.you.bots.iter().filter(|bot| bot.alive).map(|bot| {
            match thread_rng().gen_range(1, 4) {
                1 => Action::CannonAction(CannonAction {
                    bot_id: bot.bot_id,
                    pos: Position { x: thread_rng().gen_range(-self.config.field_radius, self.config.field_radius),
                                    y: thread_rng().gen_range(-self.config.field_radius, self.config.field_radius)
                    }
                }),
                2 => {
                    let allowed_positions = bot.pos.positions_within(self.config.move_);
                    let chosen = thread_rng().choose(&allowed_positions).unwrap();
                    Action::MoveAction(MoveAction {
                        bot_id: bot.bot_id,
                        pos: Position { x: chosen.x, y: chosen.y }
                    })
                },
                3 => Action::RadarAction(RadarAction {
                    bot_id: bot.bot_id,
                    pos: Position { x: thread_rng().gen_range(-self.config.field_radius, self.config.field_radius),
                                    y: thread_rng().gen_range(-self.config.field_radius, self.config.field_radius)
                    }
                }),
                _ => panic!("Doesn't happen")
            }
        }).collect()
    }

    fn set_state(&mut self, config: GameConfig, you: Team, other_teams: Vec<TeamNoPosNoHp>) {
        self.config = config;
        self.you = you;
        self.other_teams = other_teams;
    }
}

pub fn from_name(name: String) -> Box<Ai> {
    match name.as_ref() {
        "random" => Box::new(RandomAi { ..Default::default() }),
        _ => panic!("Can't find an AI with name: {}", name)
    }
}

#[test]
fn test_from_name() {
    from_name("random".to_string());
}

#[test]
#[should_panic]
fn test_from_name_nonsense() {
    from_name("not an actual ai".to_string());
}

pub struct MoveAction {
    pub bot_id: u32,
    pub pos: Position
}

pub struct RadarAction {
    pub bot_id: u32,
    pub pos: Position
}

pub struct CannonAction {
    pub bot_id: u32,
    pub pos: Position
}

pub enum Action {
    CannonAction(CannonAction),
    MoveAction(MoveAction),
    RadarAction(RadarAction)
}
