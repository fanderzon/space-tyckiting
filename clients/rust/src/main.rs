#![feature(custom_derive, custom_attribute, plugin, collections, scoped)]
#![plugin(serde_macros)]
/** Rust Tyckiting client - A websocket client for a fight to kill all other bots
 *  Copyright Futurice Oy (2015)
 *
 *  This file is part of Rust Tyckiting client.
 *
 *  Rust Tyckiting client is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  Rust Tyckiting client is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with Rust Tyckiting client.  If not, see <http://www.gnu.org/licenses/>.
 */

extern crate websocket;
extern crate serde;
extern crate rustc_serialize;
extern crate docopt;
extern crate hyper;

mod incoming;
mod ai;

use std::thread;
use std::cmp::{max, min};
use std::io::Read;
use std::error::Error;

use websocket::{Receiver, Sender};
use websocket::Message as WSMessage;
use websocket::client::request::Url;
use websocket::Client;

use serde::json;

use docopt::Docopt;

use hyper::Client as HClient;

use rustc_serialize::base64::{ToBase64, MIME};

use incoming::Message;
use ai::{Ai, Action};

static USAGE: &'static str = "
tyckiting-client - a base for your AI

Usage: tyckiting-client [--host <host>] [--name <name>] [--ai <ai>] [(--port <port> | --webgame <spec>)]
       tyckiting-client [-h]


Options:
  -h, --help                Show this help message
  -H, --host <host>         Host to connect to
  -P, --port <port>         Port to connect to
  -n, --name <name>         Team's name
  -a, --ai <ai>             Select AI
  --webgame <spec>          Ask for a game, format user:pass:opponentname
";

#[derive(RustcDecodable, Debug)]
struct Args {
    flag_host: Option<String>,
    flag_port: Option<u32>,
    flag_name: Option<String>,
    flag_ai: Option<String>,
    flag_webgame: Option<String>
}

#[derive(Serialize)]
struct WebGameRequest {
    bots: Vec<String>
}

#[derive(Deserialize)]
struct WebGameResponse {
    #[allow(dead_code)] status: String,
    #[allow(dead_code)] #[serde(rename="gameId")] game_id: u32,
    host: String,
    port: u32
}

fn main() {
    let args: Args = Docopt::new(USAGE).and_then(|d| d.decode()).unwrap_or_else(|e| e.exit());
    let host = args.flag_host.unwrap_or_else(|| "localhost".to_string());
    let team_name = args.flag_name.unwrap_or_else(|| "Team Rust".to_string());
    let ai_name = args.flag_ai.unwrap_or_else(|| "random".to_string());

    let (game_host, port) = match args.flag_webgame {
        Some(webgame) => {
            let parts: Vec<&str> = webgame.split(":").collect();
            if parts.len() != 3 {
                panic!("invalid webgame spec");
            }
            let (user, pass, opponent) = (parts[0], parts[1], parts[2]);
            let auth = format!("{}:{}", user, pass).as_bytes().to_base64(MIME);
            let mut client = HClient::new();
            let request_body_string = json::to_string(&WebGameRequest { bots: vec![opponent.to_string()]}).unwrap();
            let request_body: &str = request_body_string.as_ref();
            let mut response = client.post(Url::parse(format!("http://{}/api/new", host).as_ref()).unwrap())
                .header(hyper::header::Authorization(format!("Basic {}", auth)))
                .body(request_body)
                .send().unwrap();
            let mut body = String::new();
            response.read_to_string(&mut body).unwrap();
            if !(response.status == hyper::status::StatusCode::Ok) { panic!("Couldn't login"); }
            let game_response = json::from_str::<WebGameResponse>(body.as_ref()).unwrap_or_else(|e| {panic!("Didn't get proper response, got: {}, error was: {}", body, e)});
            println!("You can spectate at http://{}:{}.", game_response.host, game_response.port);
            (game_response.host, game_response.port)
        },
        None => (host, args.flag_port.unwrap_or_else(|| 3000u32))
    };

    let url = Url::parse(format!("ws://{}:{}", game_host, port).as_ref()).unwrap();

    let request = Client::connect(url).unwrap();

    let response_option = request.send();
    let response = match response_option {
        Err(e) => {
            println!("{:?}", e.cause().unwrap().description());
            return;
        }
        Ok(response) => response
    };
    response.validate().unwrap();

    let (mut sender, mut receiver) = response.begin().split();

    // when _receive_loop goes out of scope, the thread is waited on
    let _receive_loop = thread::scoped(move || {
        let mut ai = ai::from_name(ai_name);
        for message in receiver.incoming_messages() {
            let message = match message {
                Ok(m) => m,
                Err(e) => {
                    println!("Receive loop error: {:?}", e);
                    return;
                }
            };
            match message {
                WSMessage::Close(_) => {
                    println!("Closing");
                    return;
                }
                WSMessage::Text(msg) => {
                    let message = incoming::parse_message(msg);
                    match message {
                        Message::ConnectedMessage(_) => {
                            let _ = sender.send_message(WSMessage::Text(json::to_string(& JoinMessage {
                                type_: "join".to_string(),
                                team_name: team_name.clone() // clone because compiler doesn't know that this is called just once
                            }).unwrap()));
                        },
                        Message::EndMessage(msg) => {
                            println!("Thanks for playing!");
                            match msg.winner_team_id {
                                Some(winner) => println!("Winner was {}, you are: {}", winner, msg.you.team_id),
                                None => println!("There was no winner :(")
                            };
                            return;
                        },
                        Message::EventsMessage(msg) => {
                            ai.set_state(msg.config, msg.you, msg.other_teams);
                            let _ = sender.send_message(WSMessage::Text(json::to_string(&to_actionsmessage(
                                msg.round_id, ai.respond(msg.events))).unwrap()));
                        },
                        _ => ()
                    }
                }
                _ => println!("Got {:?}. Don't know what to do with it", message)
            }
        }
    });
}

#[derive(Debug, Deserialize, Default)]
pub struct GameConfig {
    bots: i32,
    #[serde(rename="fieldRadius")] field_radius: i32,
    #[serde(rename="move")] move_: u32,
    #[serde(rename="startHp")] start_hp: i32,
    cannon: i32,
    radar: i32,
    see: i32,
    #[serde(rename="maxCount")] max_count: i32,
    #[serde(rename="loopTime")] loop_time: i32
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Position {
    x: i32,
    y: i32
}

impl Position {
    #[allow(dead_code)]
    fn distance(&self, other: Position) -> i32 {
        // see: http://keekerdc.com/2011/03/hexagon-grids-coordinate-systems-and-distance-calculations/
        // and http://www.redblobgames.com/grids/hexagons/
        max((other.x - self.x).abs(),
            max((other.y - self.y).abs(),
            ((-other.x - other.y) - (-self.x - self.y)).abs()))
    }
    #[allow(dead_code)]
    fn positions_within(&self, distance: u32) -> Vec<Position> {
        // see http://www.redblobgames.com/grids/hexagons/
        let dist = distance as i32;
        let mut ret: Vec<Position> = Vec::new();
        for dx in -dist..dist + 1 {
            for dy in (max(-dist, -dx - dist)..min(dist, -dx + dist) + 1) {
                ret.push(Position { x: self.x + dx, y: self.y + dy});
            }
        }
        ret
    }
}

#[test]
fn check_distance() {
    assert!(Position { x: 0, y: 0}.distance(Position {x: 1, y: 0}) == 1);
    assert!(Position { x: 0, y: 0}.distance(Position {x: 1, y: 3}) == 4);
    assert!(Position { x: 0, y: 0}.distance(Position {x: 0, y: 5}) == 5);
}

#[test]
fn check_positions_within() {
    let pos = Position { x: 0, y: 0};
    assert!( pos.positions_within(1).len() == 7);
    assert!( pos.positions_within(2).len() == 19);
}

#[derive(Debug, Serialize)]
struct JoinMessage {
    #[serde(rename="type")] type_: String,
    #[serde(rename="teamName")] team_name: String
}

#[derive(Debug, Serialize)]
struct SerializableAction {
    #[serde(rename="type")] type_: String,
    #[serde(rename="botId")] bot_id: u32,
    pos: Position
}

#[derive(Debug, Serialize)]
struct ActionsMessage {
    #[serde(rename="type")] type_: String,
    #[serde(rename="roundId")] round_id: u32,
    actions: Vec<SerializableAction>
}

#[allow(dead_code)]
fn to_actionsmessage(round_id: u32, actions: Vec<Action>) -> ActionsMessage {
    let serializable_actions = actions.into_iter().map(|action| {
        match action {
            Action::RadarAction(a) => SerializableAction {type_: "radar".to_string(), bot_id: a.bot_id, pos: a.pos},
            Action::MoveAction(a) => SerializableAction {type_: "move".to_string(), bot_id: a.bot_id, pos: a.pos},
            Action::CannonAction(a) => SerializableAction {type_: "cannon".to_string(), bot_id: a.bot_id, pos: a.pos}
        }
    }).collect();
    ActionsMessage { type_: "actions".to_string(), round_id: round_id, actions: serializable_actions }
}

#[test]
fn test_actionsmessage_serialization() {
    let generated = json::to_string(&to_actionsmessage(2u32, vec![Action::CannonAction(
            ai::CannonAction {bot_id: 1u32, pos: Position { x: -1, y: 3 }})])).unwrap();
    let wanted = "{\"type\":\"actions\",\"roundId\":2,\"actions\":[{\"type\":\"cannon\",\"botId\":1,\"pos\":{\"x\":-1,\"y\":3}}]}".to_string();
    assert!( generated == wanted);
}
