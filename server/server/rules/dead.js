"use strict";

var _ = require("lodash");
var tools = require("./tools.js");

function generateEvents(actions, world) {
    return _.filter(world.bots, function (bot) {
        return bot.hp <= 0;
    });
}

function applyEvents(dead, world) {
    var bots = _.without(world.bots, dead);
    world.bots = bots;
    return world;
}

function messages(events) {
    return events.map(function (event) {
        return tools.createMessage("all", {
            event: "die",
            botId: event.botId
        });
    });
}

module.exports = {
    events: generateEvents,
    applyEvents: applyEvents,
    messages: messages
};
