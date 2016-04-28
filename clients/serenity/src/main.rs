extern crate serde;
extern crate serde_json;
extern crate websocket;

use std::str::from_utf8;
use websocket::client::request::Url;
use websocket::{Client, Message, Sender, Receiver};
use websocket::message::Type;

mod defs;

fn main() {
    let addr = "ws://localhost:3000".to_string();
    let agent = "rust-websocket";
    let game_host = "localhost";
    let game_port = "3000";

    println!("Using fuzzingserver {}", addr);
    println!("Using agent {}", agent);

    let mut game_on = true;

    while game_on {
        let url = Url::parse(format!("ws://{}:{}", game_host, game_port).as_ref()).unwrap();
        let request = Client::connect(url).unwrap();
        let response = request.send().unwrap();
        match response.validate() {
            Ok(()) => (),
            Err(e) => {
                println!("{:?}", e);
                return;
            }
        }

        let (mut sender, mut receiver) = response.begin().split();
        for message in receiver.incoming_messages() {
            let message: Message = match message {
                Ok(message) => message,
                Err(e) => {
                    println!("Error: {:?}", e);
                    let _ = sender.send_message(&Message::close());
                    game_on = false;
                    break;
                }
            };

            if !handle_message(&mut sender, message) {
                game_on = false;
                break;
            }
        }
    }
}

fn handle_message<S: Sender>(sender: &mut S, message: Message) -> bool {
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
                    let start_json: defs::IncomingStart = serde_json::from_str(&pl).unwrap();
                    println!("Got start message!");
                }
                _ => ()
            }

            // Why are we doing this?
            let response = Message::text(from_utf8(&*message.payload).unwrap());
            let _ = sender.send_message(&response).unwrap();
        }
        Type::Binary => {
            println!("It's binary!");
            let _ = sender.send_message(&Message::binary(message.payload)).unwrap();
        }
        Type::Close => {
            println!("It's a close message, exiting");
            let _ = sender.send_message(&Message::close());
            return false;
        }
        Type::Ping => {
            println!("It's a ping!");
            let _ = sender.send_message(&Message::pong(message.payload)).unwrap();
        }
        _ => (),
    }
    // Keep playing
    return true;
}

fn send_join_message<S: Sender>(sender: &mut S) {
    let join_msg = defs::JoinMessage { event_type: "join".to_string(), team_name: "Serenity".to_string() };
    let join_string = serde_json::to_string(&join_msg).unwrap();
    let join_message = Message::text( join_string.to_string() );
    sender.send_message(&join_message).unwrap();
}
