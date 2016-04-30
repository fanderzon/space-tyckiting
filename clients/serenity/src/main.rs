extern crate serde; extern crate serde_json;
extern crate websocket;
extern crate rand;

use std::str::from_utf8;
use websocket::client::request::Url;
use websocket::{Client, Message, Sender, Receiver};
use websocket::message::Type;
use rand::Rng;

mod defs;

static ADDR: &'static str = "ws://localhost:3000";
static AGENT: &'static str = "rust-websocket";
static GAME_HOST: &'static str = "localhost";
static GAME_PORT: &'static str = "3000";

fn connect() -> (websocket::sender::Sender<websocket::stream::WebSocketStream>, 
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

fn main() {
    let (mut sender, mut receiver) = connect();

    let mut state = State { started: false, bots: Vec::new() };
    for message in receiver.incoming_messages() {
        let message: Message = match message {
            Ok(message) => message,
            Err(e) => {
                println!("Error: {:?}", e);
                let _ = sender.send_message(&Message::close());
                break;
            }
        };

        if !state.handle_message(&mut sender, message) {
            break;
        }
    }
}

fn send_join_message<S: Sender>(sender: &mut S) {
    let mut rng = rand::thread_rng();
    let join_msg = defs::JoinMessage { event_type: "join".to_string(), team_name: format!("Serenity{}", rng.gen::<u8>()) };
    let join_string = serde_json::to_string(&join_msg).unwrap();
    let join_message = Message::text( join_string.to_string() );
    sender.send_message(&join_message).unwrap();
}
//
// Random, doesn't care about the size of the board...
fn get_random_pos() -> defs::Pos {
    let mut rng = rand::thread_rng();
    defs::Pos { x: rng.gen::<i16>(), y: rng.gen::<i16>() }
}

struct State {
    started: bool,
    bots: Vec<Bot>,
}

impl State {
    fn handle_message<S: Sender>(&mut self, sender: &mut S, message: Message) -> bool {
        print!("Got a message... ");

        match message.opcode {
            Type::Text => {
                println!("It's text!");

                let pl = from_utf8(&message.payload).unwrap();
                let message_json: defs::IncomingMessage = serde_json::from_str(&pl).unwrap();

                match message_json.event_type.as_ref() {
                    "connected" => {
                        let connected_json: defs::IncomingConnected = serde_json::from_str(&pl).unwrap();
                        if connected_json.event_type == "connected" {
                            println!("Got connected message, sending join.");
                            send_join_message(sender);
                            println!("Now we wait for start.");
                        }
                    }
                    "start" => {
                        println!("Got start message!");
                        self.started = true;
                        let start_json: defs::Start = serde_json::from_str(&pl).unwrap();
                        self.init(&start_json);
                    }
                    "events" => {
                        println!("Got som events!");
                        let event_json: defs::IncomingEvents = serde_json::from_str(&pl).unwrap();
                        self.send_actions_message(sender, &event_json);

                    }
                    "end" => {
                        println!("Got end message, we're ending!");
                        return false;
                    }
                    ev => {
                        println!("Got unrecognized event type {}, ignoring.", ev);
                    }
                }
            }
            _ => {
                println!("Got a weird non-text message from server, ignoring.");
            }
        }
        // Keep playing
        return true;
    }

    fn init(&mut self, start_json: &defs::Start) {
        for bot_def in &start_json.you.bots {
            self.bots.push(Bot::new(&bot_def));
        }
    }

    fn send_actions_message<S: Sender>(&self, sender: &mut S, events: &defs::IncomingEvents) {
        let stupid_actions = defs::ActionsMessage {
            event_type: "actions".to_string(),
            round_id: events.round_id,
            actions: self.bots.iter().map(|bot| defs::Action{
                bot_id: bot.id,
                action_type: "radar".to_string(),
                pos: get_random_pos()
            }).collect(),
        };
        let actions_string = serde_json::to_string(&stupid_actions).unwrap();
        let actions_message = Message::text( actions_string.to_string() );
        sender.send_message(&actions_message).unwrap();
        println!("Sending some random messages");
    }
}

struct Bot {
    id: i16,
    name: String,
    alive: bool,
    pos: defs::Pos,
    hp: i16,
}

impl Bot {
    fn new(def: &defs::Bot) -> Bot {
        return Bot {
            id: def.bot_id,
            name: def.name.to_owned(),
            alive: def.alive,
            pos: def.pos.unwrap(),
            hp: def.hp.unwrap(),
        };
    }
}


