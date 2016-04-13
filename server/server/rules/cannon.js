"use strict";

var _ = require("lodash");
var tools = require("./tools.js");
var position = require("../position.js");

function generateEvents(actions, world, rules) {
    return _.flatten(tools.filterActionsByType(actions, "cannon").map(_.partial(handleBlast, world.bots, rules.fieldRadius, rules.cannon)));
}

function applyEvents(events, world) {
    events.forEach(function (cannon) {
        _.where(world.bots, { botId: cannon.target.botId }).map(function (player) {
            player.hp -= cannon.damage;
        });
    });

    return world;
}

function handleBlast(bots, fieldRadius, blastRadius, action) {
    // radar out-of-bounds, do nothing
    if (position.distance(position.origo, action.pos) > fieldRadius) {
        return [];
    }

    return _.chain(bots)
        .map(function (bot) {
            var distance = position.distance(bot.pos, action.pos);
            if (distance <= blastRadius) {
                return {
                    source: _.findWhere(bots, { botId: action.botId }),
                    target: bot,
                    damage: 1 + blastRadius - distance
                };
            }
        })
        .compact()
        .value();
}

function messages(events) {
    return events.reduce(function (memo, event) {
        return memo.concat(hitMessage(event)).concat(damageMessage(event));
    }, []);
}

function damageMessage(event) {
    return tools.createMessage(event.target.teamId, {
        event: "damaged",
        botId: event.target.botId,
        damage: event.damage
    });
}

function hitMessage(event) {
    return tools.createMessage(event.source.teamId, {
        event: "hit",
        source: event.source.botId,
        botId: event.target.botId
    });
}

module.exports = {
    events: generateEvents,
    applyEvents: applyEvents,
    messages: messages
};
