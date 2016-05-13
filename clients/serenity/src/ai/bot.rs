use defs;
use position::Pos;

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
}
