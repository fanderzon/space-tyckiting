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
    // Abstraction for the attacking methods to use
    // They pass in the number of available bots and this method will use the
    // right spread strategy for that number and return a vector
    // TODO: Implement actual smart shooting
    pub fn smart_attack_spread(&self, available_bots: i16) -> Vec<Pos> {
        let mut shoot_at: Vec<Pos> = Vec::new();

        match available_bots {
            // If 4, shoot smart with 3 of them and randomly with the 4th
            4 => {
                shoot_at = self.triangle_smart();
                shoot_at.push(*self);
            },
            3 => shoot_at = self.triangle_smart(),
            2 => {
                //TODO: Choose twin based on pos in map.
                shoot_at = self.rand_twin();
            }
            1 => {
                shoot_at.push(self.random_spread());
            },
            _ => ()
        }
        shoot_at
    }

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

    pub fn rand_twin(&self) -> Vec<Pos> {
        let ori = match rand::thread_rng().gen_range(0, 3) {
            0 => Orientation::Horizontal,
            1 => Orientation::Slash,
            _ => Orientation::Backslash,
        };
        return self.twin(ori);
    }

    pub fn twin(&self, orientation: Orientation) -> Vec<Pos> {
        let x = self.x;
        let y = self.y;
        return match orientation {
            Orientation::Horizontal => vec![ Pos::new(x+1, y  ),
                                             Pos::new(x-1, y  ) ],
            Orientation::Slash      => vec![ Pos::new(x+1, y-1),
                                             Pos::new(x-1, y+1) ],
            Orientation::Backslash  => vec![ Pos::new(x,   y-1),
                                             Pos::new(x,   y+1) ],
        };
    }
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Slash,
    Backslash,
}
