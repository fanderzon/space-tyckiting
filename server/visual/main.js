/*global define, WebSocket, window*/
"use strict";

var HEX_SIZE = 18;
var messageBox = new MessageBox();
var ui = new Ui();
var grid = null;

var teamIndex = 1;
var bot = {
  botId: 1
};

function displayRadar( grid, point ) {
  grid.radar(bot.botId, point.x, point.y, teamIndex);
  return point;
}

var radarPoints = [];
var rowStart = 0;
var rowEnd = config.fieldRadius - config.radar;
var rowStartChange = -( config.radar + 1);
var rowEndChange = 0;

function fillRadarPoints( num ) {
  while(radarPoints.length < num) {
    radarPoints.push( displayRadar(grid, findRadarPoint()) );
  }
}

function findRadarPoint() {
  //
  var lastPoint = radarPoints[ radarPoints.length - 1 ];
  if (!lastPoint) {
    // Start in the top left corner, since we set the center for the radar area
    // let's subtract whatever the radar radius is from the
    return new Point( rowStart, -(config.fieldRadius - config.radar) );
  }

  console.log( 'lastPoint', lastPoint, rowEnd, rowEndChange );
  if (lastPoint.x >= rowEnd ) {
    rowStart = rowStart + rowStartChange;
    rowEnd = rowEnd + rowEndChange;

    if (rowStart <= -(config.fieldRadius - config.radar)) {
      console.log('too low rowStart', rowStart);
      rowStart = -(config.fieldRadius - config.radar);
      rowStartChange = 0;
      rowEndChange = -( config.radar + 1);
      rowEnd = config.fieldRadius - ( (lastPoint.y + 4) || 0) - config.radar;
    }
    return {
      x: rowStart,
      y: lastPoint.y + ( config.radar + 1 )
    };
  }

   return _.assign( {}, lastPoint, {
    x: lastPoint.x + 4
  });
}


grid = grid || new MainScreen("mainScreen", HEX_SIZE, config.fieldRadius, config.cannon, config.radar);

// displayRadar( grid, new Point( 0, 0 ) );



// if (content.type === "connected") {
//
//     config = content.config;
//
//     grid = grid || new MainScreen("mainScreen", HEX_SIZE, config.fieldRadius, config.cannon, config.radar);
//
//     socket.send(JSON.stringify({type: "spectate", data: {}}));
//
// } else if (content.type === "start") {
//     clearMessages();
//     grid.clearAll();
//     ui.reset();
// } else if (content.type === "round") {
//
//     grid.clear();
//
//     var actions = content.actions.reduce(function (memo, action) {
//         memo[action.botId] = action;
//         return memo;
//     }, {});
//
//     content.asteroids.forEach(function (asteroid) {
//         grid.addAsteroid(asteroid);
//     });
//
//     content.teams.forEach(function (team, teamIndex) {
//         team.bots.forEach(function (bot) {
//             if (!grid.hasShip(bot.botId)) {
//                 grid.addShip(bot.botId, bot.pos.x, bot.pos.y, teamIndex);
//             }
//             if (bot.hp <= 0) {
//                 grid.destroyShip(bot.botId);
//             } else {
//                 grid.moveShip(bot.botId, bot.pos.x, bot.pos.y);
//             }
//
//             var action = actions[bot.botId];
//
//             if (action) {
//                 if (action.type === "radar") {
//                     grid.radar(bot.botId, action.pos.x, action.pos.y, teamIndex);
//                 } else if (action.type === "cannon") {
//                     grid.blast(bot.botId, action.pos.x, action.pos.y, teamIndex);
//                 }
//             }
//
//             if (!ui.hasBot(bot.botId)) {
//                 ui.addBot(bot, teamIndex, config);
//             } else {
//                 ui.updateBot(bot);
//             }
//         });
//     });
// } else if (content.type === "end") {
//     showMessage("Server", "server", "Game ended. " + content.winner + " won!", true);
// }


function clearMessages() {
    messageBox.clear();
}

function showMessage(source, id, message, friend) {
    messageBox.addMessage(source, id, message, friend ? "friend" : "foe");
}
