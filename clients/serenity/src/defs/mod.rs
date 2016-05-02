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

pub fn parse_event(ev: &SomeEvent) -> Event {
    match ev.event.as_ref() {
        HIT => {
            return Event::Hit(HitEvent{
                bot_id: ev.bot_id.unwrap(),
                source: ev.source.unwrap(),
            });
        }
        DIE => {
            return Event::Die(DieEvent{
                bot_id: ev.bot_id.unwrap(),
            });
        }
        SEE => {
            return Event::See(SeeEvent{
                bot_id: ev.bot_id.unwrap(),
                source: ev.source.unwrap(),
                pos: ev.pos.unwrap(),
            });
        }
        ECHO => {
            return Event::Echo(EchoEvent{
                pos: ev.pos.unwrap(),
            });
        }
        DETECTED => {
            return Event::Detected(DetectedEvent{
                bot_id: ev.bot_id.unwrap(),
            });
        }
        DAMAGED => {
            return Event::Damaged(DamagedEvent{
                bot_id: ev.bot_id.unwrap(),
                damage: ev.damage.unwrap(),
            });
        }
        MOVE => {
            return Event::Move(MoveEvent{
                bot_id: ev.bot_id.unwrap(),
                pos: ev.pos.unwrap(),
            });
        }
        NOACTION => {
            return Event::Noaction(NoactionEvent{
                bot_id: ev.bot_id.unwrap(),
            });
        }
        _ => {
            return Event::Invalid;
        }
    }
}

