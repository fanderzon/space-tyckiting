"use strict";

var _ = require("lodash");
var chalk = require("chalk");
var position = require("../../position.js");
var radar = require('./radar.js');

// Updated at begginning of each makeDecisions, global state that every function can read from.
var state = {
    players : null,
    gameMap: [],
    radarPoints: [],
    lastRadarPoint: null,
    config: null,
    botsAlive: 0,
    lastPing: {
      roundId: 0,
      position: null
    },
    lastTarget: null
};

function getRadarPoint() {
  // Really confusing ternary for "if last radar point is null or out of bounds set, to 0, otherwise +1 on last radar point"
  var radarPoint;
  if ( state.lastRadarPoint === null ) {
    console.log('state.lastRadarPoint is null');
    radarPoint = 0;
  } else if ( state.lastRadarPoint + 1 > state.radarPoints.length - 1 ) {
    console.log( 'state.lastRadarPoint + 1 is larger than the state.radarPoints.length');
    radarPoint = 0;
  } else {
    console.log('setting radarpoint to +1 of state.lastRadarPoint');
    radarPoint = state.lastRadarPoint + 1;
  }

  state.lastRadarPoint = radarPoint;
  console.log('state.lastRadarPoint', state.lastRadarPoint, radarPoint, state.radarPoints.length);
  return state.radarPoints[radarPoint];
}

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

// Converts an object with x and y to a position representing those coordinates.
function pos(obj) {
    return position.make(obj.x, obj.y);
}

// Returns three points in a triangle around the given point.
function shootPoints(x, y) {
    return [position.make(x-1, y+2),
            position.make(x+2, y-1),
            position.make(x-1, y-1),]
}

function distanceToOurNearestBot(pos) {

}

// Plans to do an evade action, which is moving as far as possible, in the direction
// away from our other bots.
function evade( plannedActions, player ) {
  var maxMoves = maxDistanceMoves( player, state.config.move, state.config.fieldRadius );

  // Finding move which avoids our other bots the most
  var mostAvoidingMove = -1;
  var moveDistToNearestBot = -1;
  for (var i = 0; i < maxMoves.length; i++) {
      var move = maxMoves[i]

      // Finding the closest player to this specific move
      var smallestDistToOtherPlayer = 1000;
      Object.keys(state.players).forEach(function(key) {
          var otherBot = state.players[key]

          // Exclude the bot we are evading with
          if (otherBot == player) {
              return;
          }

          var dist = position.distance(pos(move), pos(otherBot));
          if (dist < smallestDistToOtherPlayer) {
              smallestDistToOtherPlayer = dist;
          }
      });

      if (smallestDistToOtherPlayer > moveDistToNearestBot) {
          moveDistToNearestBot = smallestDistToOtherPlayer;
          mostAvoidingMove = i;
      }
      console.log(chalk.blue("Move to" + JSON.stringify(maxMoves[mostAvoidingMove]) + " Had distance " + moveDistToNearestBot))
  }

  var evadePos = maxMoves[mostAvoidingMove];

  console.log(chalk.blue("Evaded to " + JSON.stringify(evadePos) +
    "because out bots were at " +
    _.map(state.players, function(player) {return "(" + player.x + ", " + player.y + ")";})));

  plannedActions[player.botId] = {
    mode: "EVADE",
    action: prepareAction(player.move, evadePos.x, evadePos.y)
  };
}

module.exports = function Ai() {

  function planForAttack(plannedActions, players, x, y) {
    return _.reduce(plannedActions, function(result, value, key) {
      var player = state.players[key];
      if (value.mode === "EVADE") {
        result[key] = value;
      } else {
        var shootPoint = position.make(x, y);

        if (Math.random() > (1/state.botsAlive)) {
            var potentialShootPoints = shootPoints(x, y);
            shootPoint = potentialShootPoints[player.incrementalID % potentialShootPoints.length]
        }

        result[key] = {
          mode: "ATTACK",
          action: prepareAction(players[key].cannon, shootPoint.x, shootPoint.y)
        };
      }
      return result;
    }, {});
  }

  function isOurBot(botId) {
      return _.reduce(state.players, function(acc, current) {
          return acc || current.botId === botId;
      }, false);
  }

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

    // Set config to state once
    if ( !state.config ) {
      state.config = config;
    }

    var fieldRadius = config.fieldRadius;
    var maxMove = config.move;

    // Let's set the game map as an array for easy random positions
    if (state.gameMap.length === 0) {
      state.gameMap = position.neighbours( position.origo, config.fieldRadius );
      state.gameMap.push( position.origo );
    }

    if ( state.radarPoints.length === 0 ) {
      state.radarPoints = radar.getRadarPoints( config );
    }

    // Map bot to id, for easier usage
    var players = _.reduce(bots, function(memo, bot) {
      memo[bot.botId] = bot;
      return memo;
    }, {});

    console.log(chalk.blue(JSON.stringify(players)));

    // Give each player an incremental ID to every player, mostly for use with shooting triangles.
    var i = 0;
    Object.keys(players).forEach(function(key) {
        players[key].incrementalID = i;
        i++;
    });

    // Now everyone has access to players
    state.players = players;

    // Set the default action for all my alive bots to random radaring
    state.botsAlive = 0;
    var plannedActions = _.reduce(players, function(memo, player) {
      if (player.alive) {
        state.botsAlive++;
        if (state.lastPing.roundId && state.lastPing.roundId >= (roundId - 2) )  {
          var p = state.lastPing;
        }
        var p = getRadarPoint();
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
        if (!isOurBot(event.botId)) {
            console.log( 'We hit something, attack!' );
            state.lastPing = {
              roundId: roundId,
              position: state.lastTarget
            };
            plannedActions = planForAttack(plannedActions, players, state.lastTarget.x, state.lastTarget.y);
        }
      } else if (event.event === "see" || event.event === "radarEcho") {
        var pos = event.pos;
        state.lastPing = {
          roundId: roundId,
          position:pos
        };
        console.info(chalk.blue("Saw bot at " + JSON.stringify(pos)));
        plannedActions = planForAttack(plannedActions, players, pos.x, pos.y);
        state.lastTarget = _.clone(pos); // TODO: dunno if need to clone
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
    botNames: [
      "Mal",
      "Zoe",
      "Wash"
    ],
    makeDecisions: makeDecisions
  };
};
