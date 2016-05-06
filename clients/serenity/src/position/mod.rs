extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use util;
use rand;
use rand::Rng;

include!(concat!(env!("OUT_DIR"), "/position.rs"));
