extern crate serde; extern crate serde_json;
extern crate websocket;
extern crate rand;

mod defs;
mod util;
mod ai;

use std::str::from_utf8;
use websocket::{Message, Sender, Receiver};
use websocket::message::Type;
use websocket::ws::dataframe::DataFrame;
use rand::Rng;
use ai::Ai;

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

        match ai.handle_message(message) {
            Ok(actions) => {
                let actions_string = serde_json::to_string(&actions).unwrap();
                let actions_message = Message::text( actions_string.to_string() );
                sender.send_message(&actions_message).expect("Sending actions message failed.");
            }
            Err(do_what) => {
                if do_what == ai::What_to_do::Exit {
                    break;
                }
            }
        }
    }
}

// Handshake waits fora connected message, sends a join then returns.
// It is intented that the is past the connected message afterwards.
fn handshake<S: Sender>(sender: &mut S, message: &Message) {
    if message.opcode == Type::Text {
        let pl = from_utf8(&message.payload).unwrap();
        let message_json: defs::IncomingMessage = serde_json::from_str(&pl).unwrap();
        if message_json.event_type == "connected" {
            let connected_json: defs::IncomingConnected = serde_json::from_str(&pl).unwrap();
            println!("Got connected message, sending join.");
            sender.send_message(&join_message()).expect("Sending join message failed.");;
            println!("Now we wait for start.");
            return;
        }
    }
}

// Och hur fan kompilerar det här?
fn get_start(message: Message) -> defs::Start {
    let pl = from_utf8(&message.payload).unwrap();
    let message_json: defs::IncomingMessage = serde_json::from_str(&pl).unwrap();
    let start_json: defs::Start = serde_json::from_str(&pl).unwrap();
    return start_json;
}

fn join_message<'a>() -> Message<'a> {
    let mut rng = rand::thread_rng();
    let join_msg = defs::JoinMessage { event_type: "join".to_string(), team_name: format!("Serenity{}", rng.gen::<u8>()) };
    let join_string = serde_json::to_string(&join_msg).unwrap();
    let join_message = Message::text( join_string.to_string() );
    return join_message;
}
