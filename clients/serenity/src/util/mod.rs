extern crate rand;
extern crate websocket;

use websocket::client::request::Url;
use websocket::Client;

use rand::Rng;
use position::{Pos};

static ADDR: &'static str = "ws://localhost:3000";
static AGENT: &'static str = "rust-websocket";
static GAME_HOST: &'static str = "localhost";
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

// Random, doesn't care about the size of the board...
pub fn get_random_pos(positions: &Vec<Pos>) -> Pos {
    let pos = rand::thread_rng().choose(&positions).unwrap();
    pos.clone()
    // Pos { x: rng.gen::<i16>(), y: rng.gen::<i16>() }
}
