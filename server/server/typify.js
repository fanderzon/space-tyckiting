"use strict";

var typify = require("typify");
var _ = require("lodash");

typify.alias("teamId", "nat");
typify.alias("botId", "nat");

var gameConfigSpec = {
    bots: "nat",
    fieldRadius: "nat",
    move: "nat",      // move radius, inclusive
    startHp: "nat",
    cannon: "nat",    // cannon radius, direct damage is radius + 1, damage decay linearly as function of distance
    radar: "nat",     // radar radius, inclusive
    see: "nat",       // sight radius, inclusive
    maxCount: "nat",  // game length in rounds
    asteroids: "nat", // asteroids
    loopTime: "nat",  // delay between rounds
    noWait: "boolean"    // start next round if all players have registered their actions
};

typify.record("gameConfig", gameConfigSpec, true);
typify.record("userConfig", gameConfigSpec, true);

typify.record("connectedMessage", {
    type: "'connected'",
    teamId: "teamId",
    config: "userConfig"
}, true);

typify.record("position", {
    x: "integer",
    y: "integer"
}, true);

var botSpec = {
    botId: "botId",
    name: "string",
    teamId: "teamId",
    hp: "integer", // can be negative, dead if so
    alive: "boolean",
    pos: "position"
};

var botNoPosNoHpSpec = _.omit(botSpec, "hp", "pos");

typify.record("bot", botSpec);
typify.record("botNoPosNoHp", botNoPosNoHpSpec);

var teamSpec = {
    name: "string",
    teamId: "teamId",
    bots: "array bot"
};

var teamNoPosNoHpSpec = _.extend({}, teamSpec, {
    bots: "array botNoPosNoHp"
});

typify.record("team", teamSpec, true);
typify.record("teamNoPosNoHp", teamNoPosNoHpSpec, true);

typify.record("startMessage", {
    type: "'start'",
    config: "userConfig",
    you: "team",
    otherTeams: "array teamNoPosNoHp"
}, true);

typify.record("joinMessage", {
    type: "'join'",
    teamName: "string"
}, true);

typify.record("endMessage", {
    type: "'end'",
    winnerTeamId: "teamId | integer",
    you: "team"
}, true);

// Someone hit you, you are damaged
typify.record("damagedEvent", {
    event: "'damaged'",
    botId: "botId",
    damage: "nat"
}, true);

// you hit someone
typify.record("hitEvent", {
    event: "'hit'",
    botId: "botId",
    source: "botId"
}, true);

// someone died
// The information is persisted in transmitted messages too.
typify.record("dieEvent", {
    event: "'die'",
    botId: "botId"
}, true);

// `source` saw `botId` (either they are close, or thru radar)
typify.record("seeEvent", {
    event: "'see'",
    source: "botId",
    // Should hp be visible?
    botId: "botId",
    pos: "position"
}, true);

typify.record("seeAsteroidEvent", {
    event: "'seeAsteroid'",
    pos: "position"
}, true);

typify.record("radarEchoEvent", {
    event: "'radarEcho'",
    pos: "position"
}, true);

// this bot is detected by someone
typify.record("detectedEvent", {
    event: "'detected'",
    botId: "botId"
}, true);

// this bot did nothing
typify.record("noactionEvent", {
    event: "'noaction'",
    botId: "botId"
}, true);

// Own bots movements
typify.record("moveEvent", {
    event: "'move'",
    botId: "botId",   // who moved
    pos: "position"      // new position
}, true);

typify.alias("event", "hitEvent | damagedEvent | dieEvent | seeEvent | seeAsteroidEvent | radarEchoEvent | detectedEvent | noactionEvent | moveEvent");

typify.record("eventsMessage", {
    type: "'events'",
    roundId: "nat",
    config: "userConfig",
    you: "team",
    otherTeams: "array teamNoPosNoHp",
    events: "array event"
}, true);

// Actions

// Move
// valid moves are only the neighbor cells, otherwise action in skipped.
typify.record("moveAction", {
    type: "'move'",
    botId: "botId",
    pos: "position"
}, true);

typify.record("radarAction", {
    type: "'radar'",
    botId: "botId",
    pos: "position"
}, true);

// cannon hit everyone
typify.record("cannonAction", {
    type: "'cannon'",
    botId: "botId",
    pos: "position"
}, true);

typify.alias("action", "moveAction | radarAction | cannonAction");

// array of actions can contain as many actions as it could,
// but the last per bot is taken into account
typify.record("actionsMessage", {
    type: "'actions'",
    roundId: "nat",
    actions: "array action"
}, true);

// Spectator - messages

// Start message for spectators
typify.record("spectatorStart", {
    type: "'start'",
    teams: "array team",
    config: "gameConfig"
}, true);

// Round summary for spectators
typify.record("roundSummary", {
    type: "'round'",
    roundId: "nat",
    config: "gameConfig",
    asteroids: "array position",
    teams: "array team",
    actions: "array action",
    events: "array event"
}, true);

typify.record("endSummary", {
    type: "'endSummary'",
    teams: "array team",
    winnerTeamId: "teamId | null"
}, true);

module.exports = typify;
