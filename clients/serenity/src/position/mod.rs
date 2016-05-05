extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use rand;
use rand::Rng;

include!(concat!(env!("OUT_DIR"), "/position.rs"));

impl Pos {
    pub fn triangle_smart(&self) -> Vec<Pos> {
        let mut triangle = match rand::thread_rng().gen_range(0, 2) {
            0 => { self.triangle_down() }
            _ => { self.triangle_up() }
        };
        // Shuffle so that the same will not be middled every time
        let mut rng = rand::thread_rng();
        rng.shuffle(&mut triangle[..]);
        {
            let p: &mut Pos = triangle.first_mut().expect("There whould be three points here!");
            p.x = self.x;
            p.y = self.y;
        }
        // Shuffle so that the same bot will not get the middled pos every time
        rng.shuffle(&mut triangle[..]);
        return triangle;
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
