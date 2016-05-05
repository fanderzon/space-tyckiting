extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use rand;
use rand::Rng;

include!(concat!(env!("OUT_DIR"), "/position.rs"));

impl Pos {
    pub fn triangle_smart(&self) -> Vec<Pos> {
        return match rand::thread_rng().gen_range(0, 2) {
            0 => { self.triangle_down() }
            _ => { self.triangle_up() }
        }
//        let rng = rand::thread_rng();
//        rng.shuffle(triangle);
//        let mpos = rng.choose(triangle);
//        mpos.x = self.x;
//        mpos.y = self.y;
//        return triangle;
    }

    // TODO: Generalize for shot radius
    pub fn triangle_down(&self) -> Vec<Pos> {
        let x = self.x;
        let y = self.y;
        return vec![ Pos::new(x-1, y+2),
                     Pos::new(x+2, y-1),
                     Pos::new(x-1, y-1) ];
    }

    // TODO: Generalize for shot radius
    pub fn triangle_up(&self) -> Vec<Pos> {
        let x = self.x;
        let y = self.y;
        return vec![ Pos::new(x+1, y-2),
                     Pos::new(x-2, y+1),
                     Pos::new(x+1, y+1) ];
    }
}
