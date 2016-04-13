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
use super::{Position, GameConfig};
use serde::json::{self, Value};

macro_rules! getString {
    ($obj:ident, $field:expr) => {
        String::from_str($obj.get($field).unwrap().as_string().unwrap())
    }
}

pub fn parse_message(msg: String) -> Message {
    let value: Value = json::from_str(&msg).unwrap();
    let obj = value.as_object().unwrap();
    let message = match getString!(obj, "type").as_ref() {
        "connected" => Message::ConnectedMessage(json::from_str(&msg).unwrap()),
        "start" => Message::StartMessage(json::from_str(&msg).unwrap()),
        "end" => Message::EndMessage(json::from_str(&msg).unwrap()),
        "events" => Message::EventsMessage(parse_events_message(&value)),
        _ => panic!("Couldn't parse message")
    };
    message
}

fn parse_events_message(value: &Value) -> EventsMessage {
    let obj = value.as_object().unwrap();
    EventsMessage {
        type_: getString!(obj, "type"),
        round_id: obj.get("roundId").unwrap().as_u64().unwrap() as u32,
        config: json::value::from_value(obj.get("config").unwrap().clone()).unwrap(),
        you: json::value::from_value(obj.get("you").unwrap().clone()).unwrap(),
        other_teams: json::value::from_value(obj.get("otherTeams").unwrap().clone()).unwrap(),
        events: obj.get("events").unwrap().clone().as_array().unwrap().iter().map(|ev| parse_event(ev.clone())).collect()
    }
}

fn parse_event(value: Value) -> Event {
    let value1 = value.clone();
    let obj = value1.as_object().unwrap();
    match getString!(obj, "event").as_ref() {
        "damaged" => Event::DamagedEvent(json::value::from_value(value).unwrap()),
        "hit" => Event::HitEvent(json::value::from_value(value).unwrap()),
        "die" => Event::DieEvent(json::value::from_value(value).unwrap()),
        "see" => Event::SeeEvent(json::value::from_value(value).unwrap()),
        "seeAsteroid" => Event::SeeAsteroidEvent(json::value::from_value(value).unwrap()),
        "radarEcho" => Event::RadarEchoEvent(json::value::from_value(value).unwrap()),
        "detected" => Event::DetectedEvent(json::value::from_value(value).unwrap()),
        "noaction" => Event::NoActionEvent(json::value::from_value(value).unwrap()),
        "move" => Event::MoveEvent(json::value::from_value(value).unwrap()),
        _ => panic!("Couldn't parse event")
    }
}

#[test]
fn test_parse_event() {
    let event_json = "{\"event\":\"noaction\",\"botId\":4}".to_string();
    let event_value: Value = json::from_str(&event_json).unwrap();
    let event_struct = parse_event(event_value);
    match event_struct {
        Event::NoActionEvent(ev) => assert!(ev.bot_id == 4),
        _ => panic!("Test failed")
    }
}


#[derive(Debug, Deserialize, Default)]
pub struct Bot {
    #[serde(rename="botId")] pub bot_id: u32,
    pub name: String,
    #[serde(rename="teamId")] pub team_id: u32,
    pub hp: i32,
    pub alive: bool,
    pub pos: Position
}

#[derive(Debug, Deserialize, Default)]
pub struct BotNoPosNoHp {
    #[serde(rename="botId")] pub bot_id: u32,
    pub name: String,
    #[serde(rename="teamId")] pub team_id: u32,
    pub hp: Option<i32>,
    pub alive: bool,
    pub pos: Option<Position>
}

#[derive(Debug, Deserialize, Default)]
pub struct Team {
    pub name: String,
    #[serde(rename="teamId")] pub team_id: u32,
    pub bots: Vec<Bot>
}

#[derive(Debug, Deserialize, Default)]
pub struct TeamNoPosNoHp {
    pub name: String,
    #[serde(rename="teamId")] pub team_id: u32,
    pub bots: Vec<BotNoPosNoHp>
}

#[derive(Debug, Deserialize)]
pub struct DamagedEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32,
    pub damage: u32
}

#[derive(Debug, Deserialize)]
pub struct HitEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32,
    pub source: u32
}

#[derive(Debug, Deserialize)]
pub struct DieEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32
}

#[derive(Debug, Deserialize)]
pub struct SeeEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32,
    pub source: u32,
    pub pos: Position
}

#[derive(Debug, Deserialize)]
pub struct SeeAsteroidEvent {
    event: String,
    pub pos: Position
}

#[derive(Debug, Deserialize)]
pub struct RadarEchoEvent {
    event: String,
    pub pos: Position
}

#[derive(Debug, Deserialize)]
pub struct DetectedEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32
}

#[derive(Debug, Deserialize)]
pub struct NoActionEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32
}

#[derive(Debug, Deserialize)]
pub struct MoveEvent {
    event: String,
    #[serde(rename="botId")] pub bot_id: u32,
    pub pos: Position
}

#[derive(Debug, Deserialize)]
pub enum Event {
    DamagedEvent(DamagedEvent),
    HitEvent(HitEvent),
    DieEvent(DieEvent),
    SeeEvent(SeeEvent),
    SeeAsteroidEvent(SeeEvent),
    RadarEchoEvent(RadarEchoEvent),
    DetectedEvent(DetectedEvent),
    NoActionEvent(NoActionEvent),
    MoveEvent(MoveEvent)
}

#[derive(Debug, Deserialize)]
struct ConnectedMessage {
    #[serde(rename="type")] type_: String,
    #[serde(rename="teamId")] team_id: u32,
    config: GameConfig
}

#[derive(Debug, Deserialize)]
pub struct StartMessage {
    #[serde(rename="type")] type_: String,
    pub config: GameConfig,
    pub you: Team,
    #[serde(rename="otherTeams")] pub other_teams: Vec<TeamNoPosNoHp>
}

#[derive(Debug, Deserialize)]
pub struct EndMessage {
    #[serde(rename="type")] type_: String,
    #[serde(rename="winnerTeamId")] pub winner_team_id: Option<u32>,
    pub you: Team
}

#[derive(Debug, Deserialize)]
pub struct EventsMessage {
    #[serde(rename="type")] type_: String,
    #[serde(rename="roundId")] pub round_id: u32,
    pub config: GameConfig,
    pub you: Team,
    #[serde(rename="otherTeams")] pub other_teams: Vec<TeamNoPosNoHp>,
    pub events: Vec<Event>
}

#[derive(Debug, Deserialize)]
pub enum Message {
    ConnectedMessage(ConnectedMessage),
    StartMessage(StartMessage),
    EndMessage(EndMessage),
    EventsMessage(EventsMessage)
}
