extern crate serde;
extern crate serde_json;

use position::{Pos};
use strings::{ HIT, DIE, SEE, ECHO, DETECTED, DAMAGED, MOVE, NOACTION }; 

include!(concat!(env!("OUT_DIR"), "/defs.rs"));

pub enum Event {
    Hit(HitEvent),
    Die(DieEvent),
    See(SeeEvent),
    Echo(EchoEvent),
    Detected(DetectedEvent),
    Damaged(DamagedEvent),
    Move(MoveEvent),
    Noaction(NoactionEvent),
    Invalid,
}

pub fn parse_event(ev: &String) -> Event {
    let general_json: SomeEvent = serde_json::from_str(ev).unwrap();
    match general_json.event.as_ref() {
        HIT => {
            let specific_event: HitEvent = serde_json::from_str(ev).unwrap();
            return Event::Hit(specific_event);
        }
        DIE => {
            let specific_event: DieEvent = serde_json::from_str(ev).unwrap();
            return Event::Die(specific_event);
        }
        SEE => {
            let specific_event: SeeEvent = serde_json::from_str(ev).unwrap();
            return Event::See(specific_event);
        }
        ECHO => {
            let specific_event: EchoEvent = serde_json::from_str(ev).unwrap();
            return Event::Echo(specific_event);
        }
        DETECTED => {
            let specific_event: DetectedEvent = serde_json::from_str(ev).unwrap();
            return Event::Detected(specific_event);
        }
        DAMAGED => {
            let specific_event: DamagedEvent = serde_json::from_str(ev).unwrap();
            return Event::Damaged(specific_event);
        }
        MOVE => {
            let specific_event: MoveEvent = serde_json::from_str(ev).unwrap();
            return Event::Move(specific_event);
        }
        NOACTION => {
            let specific_event: NoactionEvent = serde_json::from_str(ev).unwrap();
            return Event::Noaction(specific_event);
        }
        _ => {
            return Event::Invalid;
        }
    }
}

