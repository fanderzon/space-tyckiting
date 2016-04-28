extern crate websocket;

use std::str::from_utf8;
use websocket::client::request::Url;
use websocket::{Client, Message, Sender, Receiver};
use websocket::message::Type;

static ADDR: &'static str = "ws://localhost:3000";
static AGENT: &'static str = "rust-websocket";
static GAME_HOST: &'static str = "localhost";
static GAME_PORT: &'static str = "3000";

fn main() {
    println!("Using fuzzingserver {}", ADDR);
    println!("Using agent {}", AGENT);

    let mut game_on = true;

    while game_on {
        let url = Url::parse(format!("ws://{}:{}", GAME_HOST, GAME_PORT).as_ref()).unwrap();
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
            let message_json: AnyMessage = serde_json::from_str(&pl).unwrap();
            match message_json.event_type.as_ref() {
                "connected" => {
                    let connected_json: Connected = serde_json::from_str(&pl).unwrap();
                    println!("Got connected message, sending join.");
                    send_join_message(sender);
                    println!("Now we wait for start.");
                }
                "start" => {
                    let start_json: Start = serde_json::from_str(&pl).unwrap();
                    println!("Got start message!");
                }
                "events" => {
                    println!("Got som events!");

                }
                "end" => {
                    let end_json: End = serde_json::from_str(&pl).unwrap();
                    println!("Got end message, we're ending!");
                    return false;
                }
                ev => {
                    println!("Got unrecognized event type {}, ignoring.", ev);
                }
            }
        }
        Type::Close => {
            println!("It's a close message, exiting");
            let _ = sender.send_message(&Message::close());
            return false;
        }
        _ => {
            println!("Got a weird non-text message from server, ignoring.");
        }
    }
    // Keep playing
    return true;
}

fn send_join_message<S: Sender>(sender: &mut S) {
    let join_msg = Join { event_type: "join".to_string(), team_name: "Serenity".to_string() };
    let join_string = serde_json::to_string(&join_msg).unwrap();
    let join_message = Message::text( join_string.to_string() );
    sender.send_message(&join_message).unwrap();
}

#[derive(Serialize, Deserialize, Debug)]
struct AnyMessage {
    #[serde(rename="type")]
    event_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Connected  {
    #[serde(rename="teamId")]
    team_id: i8,
    config: Config,
}

#[derive(Serialize, Deserialize, Debug)]
struct Start {
    you: TeamSpec,
    config: Config,
    #[serde(rename="otherTeams")]
    others: Vec<TeamSpec>,
}

#[derive(Serialize, Deserialize, Debug)]
struct End {
    #[serde(rename="winnerTeamId")]
    winner_id: i8,
    you: TeamSpec,
}

#[derive(Serialize, Deserialize, Debug)]
struct TeamSpec {
    name: String,
    #[serde(rename="teamId")]
    team_id: i8,
    bots: Vec<Bot>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Bot {
    name: String,
    #[serde(rename="botId")]
    bot_id: i8,
    #[serde(rename="teamId")]
    team_id: i8,
    alive: bool,
    hp: Option<i8>,
    pos: Option<Point>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i8,
    y: i8,
}

#[derive(Serialize, Deserialize, Debug)]
struct Config  {
    bots: i8,
    #[serde(rename="fieldRadius")]
    field_radius: i8,
    #[serde(rename="move")]
    moves_allowed: i8,
    #[serde(rename="startHp")]
    start_hp: i8,
    cannon: i8,
    radar: i8,
    #[serde(rename="maxCount")]
    max_count: i16,
    asteroids: i8,
    #[serde(rename="loopTime")]
    loop_time: i16,
    #[serde(rename="noWait")]
    no_wait: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Join  {
    #[serde(rename="type")]
    event_type: String,
    #[serde(rename="teamName")]
    team_name: String,
}
