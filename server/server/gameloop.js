"use strict";

var _ = require("lodash");

function gameloop(counter, actions, world, rules, config) {
    return rules.reduce(function (state, rule) {
        var ruleEvents = [];
        var newState = state.world;
        var newMessages = [];

        if (_.isFunction(rule.events)) {
            ruleEvents = rule.events(actions, newState, config, counter);
        }
        if (_.isFunction(rule.applyEvents)) {
            newState = rule.applyEvents(ruleEvents, newState, config);
        }
        if (_.isFunction(rule.messages)) {
            newMessages = rule.messages(ruleEvents, newState, config);
        }
        return {
            world: newState,
            events: state.events.concat(ruleEvents),
            messages: state.messages.concat(newMessages)
        };
    }, { world: world, events: [], messages: []});
}

module.exports = gameloop;
