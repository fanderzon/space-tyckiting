extern crate serde;
extern crate serde_json;

use std::cmp;
use std::ops::Add;
use rand;
use rand::Rng;

use util;

include!(concat!(env!("OUT_DIR"), "/position.rs"));
