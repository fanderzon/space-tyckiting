extern crate rand;
extern crate websocket;

use websocket::client::request::Url;
use websocket::Client;
use std::collections::HashSet;
use std::hash::Hash;

use rand::Rng;
use position::{Pos};

static ADDR: &'static str = "ws://localhost:3000";
static AGENT: &'static str = "rust-websocket";
static GAME_HOST: &'static str = "10.3.200.123";
static GAME_PORT: &'static str = "3000";

// Feel free to write a better type annotation. It's not easy.
pub fn connect() -> (websocket::sender::Sender<websocket::stream::WebSocketStream>,
                 websocket::receiver::Receiver<websocket::stream::WebSocketStream>) {
    println!("Using location {}", ADDR);
    println!("Using agent {}", AGENT);
    let url = Url::parse(format!("ws://{}:{}", GAME_HOST, GAME_PORT).as_ref()).expect("Could not parse url, wtf");
    let request = Client::connect(url).expect("Could not connect to server.");
    let response = request.send().expect("Failed sending a request to server.");
    match response.validate() {
        Ok(()) => (),
        Err(e) => {
            println!("Failed to get a response, error: {:?}", e);
        }
    }
    return response.begin().split();
}

#[allow(dead_code)]
pub fn get_random_pos(positions: &Vec<Pos>) -> Pos {
    let pos = rand::thread_rng().choose(&positions).unwrap();
    pos.clone()
    // Pos { x: rng.gen::<i16>(), y: rng.gen::<i16>() }
}

#[allow(dead_code)]
pub fn get_rand_range(min: i16, max: i16) -> i16 {
    rand::thread_rng().gen_range(min, max + 1)
}

#[allow(dead_code)]
pub fn dedup_nosort<T: Eq + Hash>(vec: &mut Vec<T>) {
    let set: HashSet<T> = vec.drain(..).collect();
    vec.extend(set.into_iter());
}
