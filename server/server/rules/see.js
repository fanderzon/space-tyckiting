"use strict";

var tools = require("./tools.js");
var position = require("../position.js");

function generateEvents(actions, world, rules) {
    // go thru all bot pairs
    var botCount = world.bots.length;
    var events = [];

    for (var i = 0; i < botCount; i++) {
        for (var j = 0; j < botCount; j++) {
            var source = world.bots[i];
            var target = world.bots[j];
            var distance = position.distance(source.pos, target.pos);

            if (source.teamId !== target.teamId && distance <= rules.see) {
                events.push({
                    source: source,
                    target: target
                });
            }
        }
    }

    return events;
}

function messages(events) {
    return events.reduce(function (memo, event) {
        return memo.concat(getRadarMessage(event));
    }, []);
}

function getRadarMessage(event) {
    return tools.createMessage(event.source.teamId, {
        event: "see",
        source: event.source.botId,
        botId: event.target.botId,
        pos: event.target.pos
    });
}

module.exports = {
    events: generateEvents,
    messages: messages
};
