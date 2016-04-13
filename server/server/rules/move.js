"use strict";

var _ = require("lodash");
var tools = require("./tools.js");
var position = require("../position.js");

function generateEvents(actions, world, rules) {
    return _.chain(tools.filterActionsByType(actions, "move"))
        .map(_.partial(handleMove, rules.move, rules.fieldRadius, world))
        .compact()
        .value();
}

function applyEvents(moves, world) {
    moves.forEach(function (move) {
        _.where(world.bots, { botId: move.target.botId }).map(function (bot) {
            bot.pos = move.pos;
        });
    });

    return world;
}

function handleMove(maxMoveLength, fieldRadius, world, action) {
    var bot = _.findWhere(world.bots, { botId: action.botId });

    if (!bot) {
        console.error("Action with no bot -- should never happen");
        return undefined;
    }

    var destination = position.clamp(action.pos, fieldRadius);

    var asteroid = _.find(world.asteroids, function (a) {
        return _.isEqual(a, destination);
    });
    if (asteroid) {
        console.warn("Trying to move into asteroid");
        return undefined;
    }

    // not actually moving
    if (position.eq(destination, bot.pos)) {
        return undefined;
    }

    // trying to move too far
    var distance = position.distance(destination, bot.pos);
    if (distance > maxMoveLength) {
        return undefined;
    }

    return {
        target: bot,
        pos: destination
    };
}

function messages(events) {
    return events.map(function (event) {
        return tools.createMessage(event.target.teamId, {
            event: "move",
            botId: event.target.botId,
            pos: event.pos
        });
    });
}

module.exports = {
    events: generateEvents,
    applyEvents: applyEvents,
    messages: messages
};
