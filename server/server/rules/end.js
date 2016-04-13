"use strict";

var _ = require("lodash");
var tools = require("./tools.js");
var eventsModule = require("../events.js");

function generateEvents(actions, world, rules, counter) {
    if (counter >= rules.maxCount) {
        console.log("Last round standings:");
        _.each(world.bots, function (bot) {
            console.log("bot", bot.name, "- hp:", bot.hp);
        });

        if (_.isEmpty(world.bots)) {
            // everyone dead
            return eventsModule.tieEvent();
        } else {
            // counting hp per team
            var teamHps = _.chain(world.bots)
                .groupBy(function (bot) { return bot.teamId; })
                .map(function (bots, teamId) {
                    return {
                        teamId: parseInt(teamId, 10),
                        hp: _.chain(bots)
                            .pluck("hp")
                            .reduce(function (x, y) { return x + y; }, 0)
                            .value()
                    };
                })
                .value();

            console.info("Team HPs at the end:");
            _.each(teamHps, function (p) {
                console.info("Team", p.teamId, ":", p.hp);
            });

            var maxHp = _.chain(teamHps).pluck("hp").max().value();
            var teamsWithMaxHp = _.filter(teamHps, function (p) {
                return p.hp === maxHp;
            });

            // If only one team have max hp:
            if (teamsWithMaxHp.length === 1) {
                return eventsModule.winningEvent(teamsWithMaxHp[0].teamId);
            } else {
                return eventsModule.tieEvent();
            }
        }
    }

    if (_.unique(_.pluck(world.bots, "teamId")).length === 1) {
        return eventsModule.winningEvent(world.bots[0].teamId);
    }
    return null;
}

function applyEvents(ev, world) {
    if (ev) {
        world.finished = true;
    }
    return world;
}

function messages(ev) {
    if (ev) {
        return tools.createMessage("all", ev);
    } else {
        return null;
    }
}

module.exports = {
    events: generateEvents,
    applyEvents: applyEvents,
    messages: messages
};
