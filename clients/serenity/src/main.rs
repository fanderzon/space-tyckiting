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

            println!("Message {:?}", message);

            match message.opcode {
                Type::Text => {
                    let pl = from_utf8(&message.payload).unwrap();

                    // First get the event_type of the message
                    let socket_msg: defs::IncomingMessage = serde_json::from_str(&pl).unwrap();
                    if socket_msg.event_type == "connected" {
                        println!("Got connected message, sending join");
                        // contains team_name, which we already know
                        // let connected_json: Connected = serde_json::from_str(&pl).unwrap();
                        // println!("Connected {:?}", connected_json);
                        let join_msg = defs::JoinMessage { event_type: "join".to_string(), team_name: "Serenity".to_string() };
                        let join_string = serde_json::to_string(&join_msg).unwrap();
                        let join_message = Message::text( join_string.to_string() );
                        sender.send_message(&join_message).unwrap();
                    } else if socket_msg.event_type == "events" {
                        println!("Got events message, sending action");

                    }

                    println!("Payload : {:?}", pl);
                }
                Type::Binary => {
                    sender.send_message(&Message::binary(message.payload)).unwrap();
                }
                Type::Close => {
                    let _ = sender.send_message(&Message::close());
                    game_on = false;
                    break;
                }
                Type::Ping => {
                    sender.send_message(&Message::pong(message.payload)).unwrap();
                }
                _ => (),
            }
        }
    }
}
