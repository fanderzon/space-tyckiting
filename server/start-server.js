"use strict";
// Starts game server and spectator server
var WebSocketServer = require("ws").Server;
var express = require("express");
var http = require("http");
var app = express();
var fs = require("fs");
var path = require("path");
var program = require("commander");
var _ = require("lodash");
var game = require("./server/game.js");

var defaultConfig = {
    bots: 3,
    fieldRadius: 14,
    move: 2,
    startHp: 10,
    cannon: 1,
    radar: 3,
    see: 2,
    maxCount: 200,
    asteroids: 0,
    loopTime: 300,
    noWait: false
};

var paramMap = {
    "ruleAsteroids": "asteroids",
    "ruleRounds": "maxCount",
    "ruleFieldRadius": "fieldRadius",
    "ruleBotCount": "bots",
    "delay": "loopTime",
    "overdrive": "noWait"
};

program
    .version("0.1.0")
    .option("-p, --port [port]", "Server port")
    .option("-k, --keep-alive", "Keep server running after game ends. ")
    .option("-l, --game-log-file [file]", "Write game log to file. ")
    .option("--rule-rounds <rounds>", "Number of rounds.", parseInt)
    .option("--rule-asteroids <asteroids>", "Number of asteroids", parseInt)
    .option("--rule-field-radius <radius>", "Field radius", parseInt)
    .option("--rule-bot-count <botcount>", "Bot count", parseInt)
    .option("-d, --delay <number>", "Loop delay", parseInt)
    .option("-o, --overdrive <bool>", "Start next round if all players ready", function (val) { return val === "true"; })
    .option("-f, --file [file]", "configuration file")
    .parse(process.argv);

/* eslint-disable no-multi-spaces */
var port = program.port || "3000";
var keepAlive = program.keepAlive;
var gameLogFile = program.gameLogFile;
/* eslint-enable no-mult-spaces */

function getParams(_params, mapping) {
    var names = Object.keys(mapping);
    return names.reduce(function (memo, name) {
        if (!_.isUndefined(_params[name])) {
            var param = {};
            var mappedName = mapping[name];
            param[mappedName] = _params[name];
            return Object.assign({}, memo, param);
        } else {
            return memo;
        }
    }, {});
}

app.use(express.static(path.join(__dirname, "/spectator")));

var configFile = {};

if (program.file) {
    configFile = JSON.parse(fs.readFileSync(program.file, "utf8"));
}

var params = getParams(program, paramMap);

var config = _.extend({}, defaultConfig, configFile, params);

console.log("*** Space Tyckiting Server ***");
console.log("Config:");
console.log(JSON.stringify(config, null, 2));
console.log("Listening at ws://localhost:" + port);
console.log("Spectate game at http://localhost:" + port);
console.log("Start your AIs now!");

var server = http.createServer(app);
server.timeout = 5000;
server.listen(port);

var wss = new WebSocketServer({server: server});

var g = game(config, keepAlive, wss, gameLogFile, function () {
    console.log("Shutting down...");
    wss.close();
    server.close();
});

g.start();
