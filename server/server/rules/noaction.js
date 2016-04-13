"use strict";

var _ = require("lodash");
var tools = require("./tools.js");

function generateEvents(actions, world) {
    return _.filter(world.bots, function (bot) {
        return !_.findWhere(actions, { botId: bot.botId });
    });
}

function messages(events) {
    return events.map(function (ev) {
        return tools.createMessage(ev.teamId, {
            event: "noaction",
            botId: ev.botId
        });
    });
}

module.exports = {
    events: generateEvents,
    messages: messages
};
