"use strict";

var _ = require("lodash");
var chalk = require("chalk");
var position = require("../../position.js");

var botNames = [
  "Drone Zero",
  "Drone One",
  "Drone Two"
];

function randInt(min, max) {
  var range = max - min;
  var rand = Math.floor(Math.random() * (range + 1));
  return min + rand;
}

module.exports = function Ai() {

  function prepareAction(action, x, y) {
    return function() {
      action(x, y);
    };
  }

  function planForAttack(plannedActions, players, x, y) {
    return _.reduce(plannedActions, function(result, value, key) {
      if (value.mode === "EVADE") {
        result[key] = value;
      } else {
        result[key] = {
          mode: "ATTACK",
          action: prepareAction(players[key].cannon, x, y)
        };
      }
      return result;
    }, {});
  }

  var lastTarget = {};
  /**
   * The mastermind bot controls all the bots at one team.
   * The logic is following:
   *  - If a bot has been hit, move it to avoid more hits
   *  - If a bot managed to hit something. Everyone tries to hit the last target
   *  - If a bot sees someone, everyone shoot the first sighting
   *  - If a bot is moved, move it's position (NOTE: In case of evading, it probably should take it's changed location into account ;) )
   *  - If no special action, do radaring
   *
   * @param events
   */
  function makeDecisions(roundId, events, bots, config) {
    var allPos = position.neighbours(position.origo, config.fieldRadius);
    allPos.push(position.origo);

    // Map bot to id, for easier usage
    var players = _.reduce(bots, function(memo, bot) {
      memo[bot.botId] = bot;
      return memo;
    }, {});

    var plannedActions = _.reduce(players, function(memo, player) {
      if (player.alive) {
        var p = allPos[randInt(0, allPos.length - 1)];
        var x = p.x;
        var y = p.y;
        memo[player.botId] = {
          mode: "RADAR",
          action: prepareAction(player.radar, p.x, p.y)
        };
      }
      return memo;
    }, {});

    events.forEach(function(event) {
      if (event.event === "damaged") {
        // someone hit us
          var maxMove = config.move;
          var player = players[event.botId];
          // TODO: fix hexgrid
          var x = player.x + maxMove - maxMove * (Math.floor(Math.random() * maxMove));
          var y = player.y + maxMove - maxMove * (Math.floor(Math.random() * maxMove));

          plannedActions[event.botId] = {
            mode: "EVADE",
            action: prepareAction(player.move, x, y)
          };
      } else if (event.event === "hit") {
        // we hit!
        plannedActions = planForAttack(plannedActions, players, lastTarget.x, lastTarget.y);
      } else if (event.event === "see" || event.event === "radarEcho") {
        var pos = event.pos;
        console.info(chalk.blue("Saw bot at " + JSON.stringify(pos)));
        plannedActions = planForAttack(plannedActions, players, pos.x, pos.y);
        lastTarget = _.clone(pos); // TODO: dunno if need to clone
      } else if (event.event === "detected") {
        console.log("event", event);

        console.log("DEBUG", "->", players[event.botId]);

        players[event.botId].message("Should I do something now?");
      }
    });

    _.each(plannedActions, function(plan) {
      plan.action.apply();
    });
  }

  return {
    botNames: botNames,
    makeDecisions: makeDecisions
  };
};
