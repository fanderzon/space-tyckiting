"use strict";

var _ = require("lodash");
var chalk = require("chalk");
var position = require("../../position.js");
var radar = require('./radar.js');

var botNames = [
  "Mal",
  "Zoe",
  "Wash"
];

var gameMap = [];
var radarPoints = [];
var gameConfig = {};

var state = {
    players : null
};

function randInt(min, max) {
  var range = max - min;
  var rand = Math.floor(Math.random() * (range + 1));
  return min + rand;
}

function randomPosition( map ) {
  return map[ randInt(0, map.length - 1) ];
}

// Returns only moves that are the maximum distance allowed, yet still within the
// radius of the grid, great for evading (I hope)
function maxDistanceMoves( start, maxMoves, maxRadius ) {
  start = start || position.make(0,0);
  return position.neighbours( start, maxMoves ).filter(
    function maxMoveFilter( pos ) {
      return position.distance( start, pos ) === maxMoves &&
        ( Math.abs(pos.x) <= maxRadius && Math.abs(pos.y) <= maxRadius );
    }
  );
}

function prepareAction(action, x, y) {
  return function() {
    action(x, y);
  };
}

function pos(obj) {
    return position.make(obj.x, obj.y);
}

// Plans to do an evade action, which is moving as far as possible, in the direction
// away from our other bots.
function evade( plannedActions, player ) {
  var maxMoves = maxDistanceMoves( player, gameConfig.move, gameConfig.fieldRadius );
  //var evadePos = maxMoves[ randInt( 0, maxMoves.length - 1 ) ];
  // Old, random

  var mostAvoidingMove = -1;
  var closestDist = -1;
  for (var i = 0; i < maxMoves.length; i++) {
      var movePos = maxMoves[i]

      var smallestDistTotherPlayer = 1000;
      for (var otherBot in state.players) {
          var playerPos = pos(player);
          var botPos = pos(otherBot);
          console.log(chalk.blue(botPos + " And I'm at " + playerPos));
          var dist = position.distance(playerPos, botPos);
          if (dist < smallestDistTotherPlayer) {
              smallestDistTotherPlayer = dist;
          }
      }

      if (smallestDistTotherPlayer < closestDist) {
          closestDist = smallestDistTotherPlayer;
          mostAvoidingMove = i;
      }
  }

  var evadePos = maxMoves[mostAvoidingMove];

  console.log(chalk.blue("Evaded to " + evadePos + "because out bots were at " + state.players))

  plannedActions[player.botId] = {
    mode: "EVADE",
    action: prepareAction(player.move, evadePos.x, evadePos.y)
  };
}

module.exports = function Ai() {

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
    gameConfig = config;

    var fieldRadius = config.fieldRadius;
    var maxMove = config.move;

    // Let's set the game map as an array for easy random positions
    if (gameMap.length === 0) {
      gameMap = position.neighbours( position.origo, config.fieldRadius );
      gameMap.push( position.origo );
    }

    if (radarPoints.length === 0) {
      console.log( 'radarPoints is empty ');
      radarPoints = radar.getRadarPoints( config );
    }

    // Map bot to id, for easier usage
    var players = _.reduce(bots, function(memo, bot) {
      memo[bot.botId] = bot;
      return memo;
    }, {});

    // Now everyone has access to players
    state.players = players;

    // Set the default action for all my alive bots to random radaring
    var plannedActions = _.reduce(players, function(memo, player) {
      if (player.alive) {
        var p = randomPosition( gameMap );
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
      var player = players[event.botId];

      if (event.event === "damaged") {
          console.log( 'We were hit, evading!' );
          return evade( plannedActions, player );
      } else if (event.event === "hit") {
        console.log( 'We hit something, attack!' );
        plannedActions = planForAttack(plannedActions, players, lastTarget.x, lastTarget.y);
      } else if (event.event === "see" || event.event === "radarEcho") {
        var pos = event.pos;
        console.info(chalk.blue("Saw bot at " + JSON.stringify(pos)));
        plannedActions = planForAttack(plannedActions, players, pos.x, pos.y);
        lastTarget = _.clone(pos); // TODO: dunno if need to clone
      } else if (event.event === "detected") {
        console.log('We were detected, evading!');
        return evade( plannedActions, player );
        // var maxMoves = maxDistanceMoves( player, maxMove, fieldRadius );
        // var evadePos = maxMoves[ randInt( 0, maxMoves.length - 1 ) ];
        //
        // plannedActions[event.botId] = {
        //   mode: "EVADE",
        //   action: prepareAction(player.move, evadePos.x, evadePos.y)
        // };
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
