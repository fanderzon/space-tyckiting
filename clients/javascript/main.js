"use strict";

var WebSocket = require("ws");
var chalk = require("chalk");
var request = require("request");

function main(host, port, botName, aiName, webgameSpec) {
  if (webgameSpec) {
    webgameSpec = webgameSpec.split(/:/);
    if (webgameSpec.length !== 3) {
      throw new Error("invalid webgame spec: user:pass:opponent");
    }

    var requestBody = JSON.stringify({
      bots: [webgameSpec[2]]
    });

    request.post({
      method: "POST",
      url: "http://" + host + ":" + port + "/api/new",
      body: requestBody,
      auth: {
        user: webgameSpec[0],
        pass: webgameSpec[1],
        sendImmediately: true
      }
    }, function (error, response, body) {
      if (error) {
        console.error("WEBGAME ERROR", error);
        return;
      }
      if (response.statusCode !== 200) {
        console.error(body);
        return;
      }

      var json = JSON.parse(body);
      console.log("Got webgame response: " + JSON.stringify(json));

      port = json.port;
      host = json.host;

      // Loop back
      console.info("Spectate at", "http://" + host + ":" + port);
      setTimeout(function () {
        main(host, port, botName, aiName);
      }, 5000);
    });
    return;
  }

  var TARGET_URL = "ws://" + host + ":" + port;

  var AI = require("./ai/" + aiName + "/ai.js");

  start(new AI());

  function start(ai) {
    // Global state:
    var bots = [];
    var opponents = [];
    var botIdMap = {};
    var playerId = null;
    var config;
    var botTeam = botName;

    // Actions
    var actions = [];

    function action(botId, type, x, y) {
      var actionData = {
        botId: botId,
        type: type,
        pos: {
          x: x,
          y: y
        }
      };
      botIdMap[botId].lastAction = actionData;
      actions.push(actionData);
    }

    function clearActions() {
      actions = [];
    }

    function move(id) {
      return function(x, y) {
        return action(id, "move", x, y);
      };
    }

    function radar(id) {
      return function(x, y) {
        return action(id, "radar", x, y);
      };
    }

    function cannon(id) {
      return function(x, y) {
        return action(id, "cannon", x, y);
      };
    }

    function messageAction(/* message */) {
      // no-op for now
    }

    // Socket
    var socket = new WebSocket(TARGET_URL);

    function socketSend(data) {
      var json = JSON.stringify(data);
      console.log("SEND", chalk.cyan("->"), chalk.gray(json));
      socket.send(json);
    }

    // Handlers
    function handleConnected(data) {
      config = data.config;
      playerId = data.teamId;

      socketSend({
        type: "join",
        teamName: botTeam
      });
    }

    function handleStart(data) {
      // It set three times, but it really doesn't matter // TODO: is it?
      opponents = data.otherTeams;
      var you = data.you;

      you.bots.forEach(function(bot, index) {
        bots[index] = {
          botId: bot.botId,
          name: bot.name,
          x: bot.x,
          y: bot.y,
          hp: config.startHp,
          last: {},
          alive: true,
          move: move(bot.botId),
          radar: radar(bot.botId),
          cannon: cannon(bot.botId),
          message: messageAction
        };

        botIdMap[bot.botId] = bots[index];
      });
    }

    function handleEnd(data) {
      if (data.winnerTeamId === null) {
        console.info(chalk.cyan("TIE GAME"));
      } else if (data.winnerTeamId === playerId) {
        console.info(chalk.green("YOU WIN"));
      } else {
        console.info(chalk.red("YOU LOSE"));
      }
    }

    function handleEvents(rawData) {
      var events = rawData.events;

      var currentRound = rawData.roundId;

      // TODO Change team -> state. Additionally(or optionally) Include opponents, and bot info
      rawData.you.bots.forEach(function(bot) {
        botIdMap[bot.botId].hp = bot.hp;
        botIdMap[bot.botId].alive = bot.alive;
        botIdMap[bot.botId].map = bot.pos;
        // Legacy:
        botIdMap[bot.botId].x = bot.pos.x;
        botIdMap[bot.botId].y = bot.pos.y;
      });

      // AI!
      ai.makeDecisions(currentRound, events, bots, config, opponents);

      socketSend({
        type: "actions",
        roundId: currentRound,
        actions: actions
      });

      // Only one set of events should be passed to the MasterMind
      clearActions();

      // Printing interesting events
      events.forEach(function(event) {
        if (event.event === "damaged") {
          console.log("Someone hit us: ", event.botId);
        } else if (event.event === "see") {
          console.log("Detected bot at", event.pos.x, event.pos.y);
        } else if (event.event === "message") {
          var friendlyMessage = event.data.source.player === playerId;
          console.info("MESSAGE", event.data.source, event.data.messageId, event.data.message, friendlyMessage);
        }
      });
    }

    // Game is event driven:
    socket.onmessage = function (rawContent) {
      var content = JSON.parse(rawContent.data);
      console.log("RECV", chalk.magenta("<-"), chalk.grey(JSON.stringify(content)));

      switch (content.type) {
        case "connected": handleConnected(content); break;
        case "start": handleStart(content); break;
        case "end": handleEnd(content); break;
        case "events": handleEvents(content); break;
        default:
          console.error("UNKNOWN MESSAGE -- " + content.type);
      }
    };

    socket.onclose = function close() {
      console.log(chalk.red("-- disconnected -- "));
    };
  }
}

module.exports = main;
