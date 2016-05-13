extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use util;
use rand;
use rand::Rng;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/position.rs"));

impl Pos {
    pub fn triangle_smart(&self) -> Vec<Pos> {
        let mut triangle = match rand::thread_rng().gen_range(0, 2) {
            0 => { self.triangle_left() }
            _ => { self.triangle_right() }
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
    fn triangle_left(&self) -> Vec<Pos> {
        let x = self.x;
        let y = self.y;
        return vec![ Pos::new(x-1, y  ),
                     Pos::new(x+1, y-1),
                     Pos::new(x,   y+1) ];
    }

    // TODO: Generalize for shot radius
    pub fn triangle_right(&self) -> Vec<Pos> {
        let x = self.x;
        let y = self.y;
        return vec![ Pos::new(x+1, y  ),
                     Pos::new(x-1, y+1),
                     Pos::new(x,   y-1) ];
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
