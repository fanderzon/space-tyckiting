// This module intended for strings that can cause miscommunication with server
// (or something similar) if misspelled. It lets our compiler work as a spell-checker.

pub const CONNECTED: &'static str = "connected";
pub const JOIN: &'static str = "join";
pub const ACTIONS: &'static str = "actions";
pub const CANNON: &'static str = "cannon";
pub const END: &'static str = "end";
pub const EVENTS: &'static str = "events";
pub const RADAR: &'static str = "radar";
pub const HIT: &'static str = "hit";
pub const DIE: &'static str = "die";
pub const SEE: &'static str = "see";
pub const ECHO: &'static str = "echo";
pub const RADARECHO: &'static str = "radarEcho";
// I actually discovered a spelling error right here.
pub const DETECTED: &'static str = "detected";
pub const DAMAGED: &'static str = "damaged";
pub const MOVE: &'static str = "move";
pub const NOACTION: &'static str = "noaction";
