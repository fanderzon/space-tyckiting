extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use util;
use rand;
use rand::Rng;
use std::fmt;

include!(concat!(env!("OUT_DIR"), "/position.rs"));

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
