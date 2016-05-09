extern crate serde;
extern crate serde_json;
extern crate websocket;
extern crate rand;

mod strings;
mod defs;
mod util;
mod ai;
mod position;
mod lists;

use std::str::from_utf8;
use websocket::{Message, Sender, Receiver};
use websocket::message::Type;
use websocket::ws::dataframe::DataFrame;
use rand::Rng;
use strings::{CONNECTED, JOIN, EVENTS, END};
use ai::Ai;
use defs::{IncomingMessage, IncomingEvents, IncomingEnd};

fn main() {
    let (mut sender, mut receiver) = util::connect();
    let mut msg_iter = receiver.incoming_messages();

    handshake(&mut sender, &msg_iter.next().unwrap().unwrap());
    let start_msg = get_start(msg_iter.next().unwrap().unwrap());

    let mut ai = Ai::new(&start_msg);

    for message in msg_iter {
        let message: Message = match message {
            Ok(message) => message,
            Err(e) => {
                println!("Error: {:?}", e);
                let _ = sender.send_message(&Message::close());
                break;
            }
        };

        match message.opcode {
            Type::Text => {
                let pl = from_utf8(&message.payload).unwrap();
                let message_json: IncomingMessage = serde_json::from_str(&pl).unwrap();

                match message_json.event_type.as_ref() {
                    EVENTS => {
                        println!("Got som events!");
                        let events_json: IncomingEvents = serde_json::from_str(&pl).unwrap();
                        let actions = ai.handle_message(events_json);
                        let actions_string = serde_json::to_string(&actions).unwrap();
                        let actions_message = Message::text( actions_string.to_string() );
                        sender.send_message(&actions_message).expect("Sending actions message failed.");
                    }
                    END => {
                        let end: IncomingEnd = serde_json::from_str(&pl).unwrap();
                        println!("Game ended!");
                        if end.you.team_id == end.winner_team_id {
                            println!("WE WON!!!");
                        } else {
                            println!("WE DIDN'T WIN!!!");
                        }
                        break;
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
    }
}

fn handshake<S: Sender>(sender: &mut S, message: &Message) {
    if message.opcode == Type::Text {
        let pl = from_utf8(&message.payload).unwrap();
        let message_json: defs::IncomingMessage = serde_json::from_str(&pl).unwrap();
        if message_json.event_type == CONNECTED {
            //let connected_json: defs::IncomingConnected = serde_json::from_str(&pl).unwrap();
            println!("Got connected message, sending join.");
            sender.send_message(&join_message()).expect("Sending join message failed.");;
            println!("Now we wait for start.");
            return;
        }
    }
}

fn get_start(message: Message) -> defs::Start {
    let pl = from_utf8(&message.payload).unwrap();
    let start_json: defs::Start = serde_json::from_str(&pl).unwrap();
    return start_json;
}

fn join_message<'a>() -> Message<'a> {
    let mut rng = rand::thread_rng();
    let join_msg = defs::JoinMessage { event_type: JOIN.to_string(), team_name: format!("Serenity{}", rng.gen::<u8>()) };
    let join_string = serde_json::to_string(&join_msg).unwrap();
    let join_message = Message::text( join_string.to_string() );
    return join_message;
}
