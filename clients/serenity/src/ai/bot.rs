use defs;
use position::Pos;
use std::fmt;

const MIN_HEALTHY_HP: i16 = 4;

#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct Bot {
    pub id: i16,
    pub name: String,
    pub alive: bool,
    pub pos: Pos,
    pub hp: i16,
}

impl Bot {
    pub fn new(def: &defs::Bot) -> Bot {
        return Bot {
            id: def.bot_id,
            name: def.name.to_owned(),
            alive: def.alive,
            pos: def.pos.unwrap(),
            hp: def.hp.unwrap(),
        };
    }

    pub fn is_healthy(&self) -> bool {
        self.hp >= MIN_HEALTHY_HP
    }
}

impl fmt::Display for Bot {
    fn fmt (&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bot {} Hp: {} alive: {} pos: {}", self.id, self.hp, self.alive, self.pos)
    }
}
