"use strict";

var tools = require("./tools.js");
var position = require("../position.js");

function generateEvents(actions, world, rules) {
    // go thru all bot & asteroid pairs
    var botCount = world.bots.length;
    var asteroidsCount = world.asteroids.length;
    var events = [];

    for (var i = 0; i < botCount; i++) {
        for (var j = 0; j < asteroidsCount; j++) {
            var source = world.bots[i];
            var target = world.asteroids[j];
            var distance = position.distance(source.pos, target);

            if (distance <= rules.see) {
                events.push({
                    source: source,
                    pos: target
                });
            }
        }
    }

    return events;
}

function messages(events) {
    return events.reduce(function (memo, event) {
        return memo.concat(getSeeAsteroidsMessage(event));
    }, []);
}

function getSeeAsteroidsMessage(event) {
    return tools.createMessage(event.source.teamId, {
        event: "seeAsteroid",
        pos: event.pos
    });
}

module.exports = {
    events: generateEvents,
    messages: messages
};
