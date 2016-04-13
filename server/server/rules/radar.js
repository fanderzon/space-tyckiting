"use strict";

var _ = require("lodash");
var tools = require("./tools.js");
var position = require("../position.js");

function generateEvents(actions, world, rules) {
    return _.flatten(tools.filterActionsByType(actions, "radar").map(_.partial(handleRadar, world.bots, world.asteroids, rules.fieldRadius, rules.radar)));
}

function handleRadar(bots, asteroids, fieldRadius, radarRadius, action) {
    // radar out-of-bounds, do nothing
    if (position.distance(position.origo, action.pos) > fieldRadius) {
        return [];
    }

    var source = _.findWhere(bots, { botId: action.botId });

    var seenBots = _.chain(bots)
        .map(function (bot) {
            var distance = position.distance(bot.pos, action.pos);

            // inside range and not own bot
            if (distance <= radarRadius && bot.teamId !== source.teamId) {
                return {
                    source: source,
                    target: bot,
                    pos: bot.pos
                };
            }
        })
        .compact()
        .value();

    var seenAsteroids = _.chain(asteroids)
        .map(function (asteroid) {
            var distance = position.distance(asteroid, action.pos);

            // inside range and not own bot
            if (distance <= radarRadius) {
                return {
                    source: source,
                    pos: asteroid
                };
            }
        })
        .compact()
        .value();

    return seenBots.concat(seenAsteroids);
}

function messages(events) {
    var radarMessages = events.map(getRadarMessage);
    var detectedMessages = _.chain(events).map(getDetectedMessage).compact().value();
    return radarMessages.concat(detectedMessages);
}

function getDetectedMessage(event) {
    if (event.target) {
        return tools.createMessage(event.target.teamId, {
            event: "detected",
            botId: event.target.botId
        });
    } else {
        return undefined;
    }
}

function getRadarMessage(event) {
    return tools.createMessage(event.source.teamId, {
        event: "radarEcho",
        pos: event.pos
    });
}

module.exports = {
    events: generateEvents,
    messages: messages
};
