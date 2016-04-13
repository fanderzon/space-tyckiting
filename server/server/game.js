"use strict";

var Messages = require("./messages.js");
var _ = require("lodash");
var loop = require("./gameloop.js");
var chalk = require("chalk");
var typify = require("./typify.js");
var position = require("./position.js");
var Bluebird = require("bluebird");
var fs = require("fs");

function checkForStart(players) {
    console.log("Starting", players.length, _.where(players, {active: true}).length, 2);
    return _.where(players, {active: true}).length >= 2;
}

function socketSend(socket, type, message) {
    if (!typify.check(type, message)) {
        console.log(JSON.stringify(message, null, 2));
    }
    typify.assert(type, message);
    return new Bluebird(function (resolve) { /* , reject */
        socket.send(JSON.stringify(message), function (error) {
            if (error) {
                // TODO: reject and recover?
                resolve(error);
            } else {
                resolve();
            }
        });
    });
}

function Round(allPlayers, nextStep, loopTime, noWait) {

    var players = _.where(allPlayers, {active: true}).map(function (player) {
        return player.teamId;
    });
    var actedPlayers = [];

    var timeout = setTimeout(function () {
        nextStep();
    }, loopTime);

    function registerAction(player) {
        actedPlayers.push(player.teamId);
        if (noWait && _.isEmpty(_.difference(players, actedPlayers))) {
            clearTimeout(timeout);
            nextStep();
        }
    }

    function clear() {
        players = null;
        actedPlayers = null;
        clearTimeout(timeout);
    }

    return {
        registerAction: registerAction,
        clear: clear
    };
}

function Game(config, keepAlive, ws, gameLogFile, onEndCallback) {
    var allPlayers = [];
    var spectators = [];
    var gameLog = [];
    var started = false;
    var finished = false;

    var sockets = [];

    var idCounter = 0;
    var botIdCounter = 0;

    var currentRound;
    /*
        The rules
        - process incoming actions
        - change the world
        - generate outcoming events

        The rules CANNOT change other rule events
        but we could add this, to make e.g. "silent gun" power-ups etc.
    */
    var rules = [
        require("./rules/noaction"),
        require("./rules/move"),
        require("./rules/cannon"),
        require("./rules/dead"),
        require("./rules/see"),
        require("./rules/seeAsteroid"),
        require("./rules/radar"),
        require("./rules/end")
    ];

    function start() {
        ws.on("connection", function (socket) {
            var teamId = idCounter++;
            var player = { teamId: teamId, socket: socket};

            sockets.push(socket);

            allPlayers.push(player);

            socket.on("close", function () {
                console.info("Socket closed");

                // Remove socket from sockets array.
                // We leave it in the players (and specators)
                // as it doesn't hurt there
                sockets = sockets.filter(function (s) {
                    return s !== socket;
                });
            });

            socket.on("message", function (rawData) {
                var data = JSON.parse(rawData);
                if (data.type === "actions") {
                    if (!typify.check("actionsMessage", data)) {
                        console.error(chalk.red("ERROR"), "missformed actions message", JSON.stringify(data, null, 2));
                        return;
                    }

                    // TODO: check that data.roundId is current round id.
                    // Need to refactor a bit to make that possible :(

                    console.log("%s: ", player.name, data.actions);
                    if (!finished) {
                        player.actions = data.actions;
                        if (currentRound) {
                            currentRound.registerAction(player);
                        }
                    }
                } else if (data.type === "spectate") {
                    if (player.active) {
                        if (socket) {
                            socket.send(JSON.stringify({type: "error", data: "Already in play"}));
                        }
                    } else {
                        spectators.push(player);
                    }
                } else if (data.type === "join") {
                    if (!typify.check("joinMessage", data)) {
                        console.error(chalk.red("ERROR"), "missformed join message", JSON.stringify(data, null, 2));
                    }

                    if (started) {
                        if (socket) {
                            socket.send(JSON.stringify({type: "error", data: "Already started"}));
                        }
                        return;
                    }
                    // Clear inactive players and spectators (players without connection)
                    allPlayers = _.filter(allPlayers, function (p) {
                        return p.socket.readyState === p.socket.OPEN;
                    });

                    player.name = data.teamName;
                    player.bots = _.range(config.bots).map(function (index) {
                        var name;
                        if (data.bots && data.bots[index]) {
                            name = data.bots[index];
                        } else {
                            name = player.name + " " + (index + 1);
                        }
                        return {
                            botId: botIdCounter++,
                            name: name
                        };
                    });
                    player.active = true;

                    if (checkForStart(allPlayers)) {
                        initialise(_.where(allPlayers, {active: true}));
                    }
                }
            });

            socketSend(socket, "connectedMessage", {
                type: "connected",
                teamId: teamId,
                config: Messages.mkUserConfig(config)
            });
        });

        function sendToPlayerTyped(player, type, msg) {
            return socketSend(player.socket, type, msg);
        }

        function sendToSpectatorsTyped(type, msg) {
            gameLog.push(msg);
            return spectators.map(function (spectator) {
                return sendToSpectatorTyped(spectator, type, msg);
            });
        }

        function sendToSpectatorTyped(player, type, msg) {
            return socketSend(player.socket, type, msg);
        }

        function writeGameLog() {
            if (!gameLogFile) {
                return;
            }

            fs.writeFile(gameLogFile, JSON.stringify(gameLog), function (err) {
                if (err) {
                    console.log("Failed to write game log to " + gameLogFile);
                } else {
                    console.log("Wrote game log to " + gameLogFile);
                }
            });
        }

        function initialise(players) {
            finished = false;

            var allPositions = position.neighbours(position.origo, config.fieldRadius);
            var shuffledPositons = _.shuffle(allPositions);

            // Initialize positions and data
            var bots = players.reduce(function (memo, player) {
                return memo.concat(player.bots.map(function (bot) {
                    return {
                        botId: bot.botId,
                        name: bot.name,
                        teamId: player.teamId,
                        hp: config.startHp,
                        alive: true,
                        pos: shuffledPositons.pop()
                    };
                }));
            }, []);

            var asteroids = [];
            _.each(_.range(config.asteroids), function () {
                asteroids.push(shuffledPositons.pop());
            });

            var startMessage = Messages.spectatorStartMessage(players, bots, config);
            sendToSpectatorsTyped("spectatorStart", startMessage);

            players.forEach(function (player) {
                sendToPlayerTyped(player, "startMessage", Messages.mkStartMessage(player, players, bots, config));
            });

            started = true;
            gameLoop(players, bots, asteroids, 0, [], 200);
        }

        function gameLoop(players, bots, asteroids, roundCounter, statistics, loopTime, noWait) {
            // TODO Does the old round need deleting?
            currentRound = new Round(players, function () {
                innerLoop(players, bots, asteroids, roundCounter, statistics);
            }, loopTime, noWait);
        }

        function innerLoop(players, bots, asteroids, roundCounter, statistics) {
            console.log("Round ", roundCounter);

            // update alive
            _.each(bots, function (bot) {
                bot.alive = bot.hp > 0;
            });

            // active bots
            var activeBots = _.filter(bots, function (bot) {
                return bot.alive;
            });
            var activeBotIds = _.pluck(activeBots, "botId");

            // Actions
            var actions = [];
            if (roundCounter > 0) {
                actions = _.chain(players)
                    .map(function (player) {
                        var botIds = _.chain(player.bots)
                            .pluck("botId")
                            .filter(function (botId) {
                                return _.contains(activeBotIds, botId);
                            })
                            .value();

                        var act = _.chain(player.actions)
                            // only one action per bot
                            .uniq(false, "botId")
                            // only own bots
                            .filter(function (action) {
                                return _.contains(botIds, action.botId);
                            })
                            .value();

                        return act;
                    })
                    .flatten()
                    .value();
            }

            var world = {
                players: players,
                bots: activeBots,
                allBots: bots,
                asteroids: asteroids
            };

            var round = loop(roundCounter, actions, world, rules, config);

            var messagesByTeam = round.messages.reduce(function (memo, message) {
                if (message) {
                    var target = _.isUndefined(message.target) ? "none" : message.target;
                    if (memo[target]) {
                        memo[target].push(message.content);
                    } else {
                        memo[target] = [message.content];
                    }
                }
                return memo;
            }, {});

            var allMessages = _.filter(round.messages, function (message) {
                return message && message.content;
            }).map(function (message) {
                return message.content;
            });

            var summaryMessage = Messages.mkRoundSummaryMessage(
                players,
                bots,
                config,
                asteroids,
                roundCounter,
                actions,
                allMessages.filter(function (message) { return message.event !== "end"; }));

            sendToSpectatorsTyped("roundSummary", summaryMessage);

            players.forEach(function (player) {
                var messages = messagesByTeam[player.teamId] || [];
                if (messagesByTeam.all) {
                    messages = messages.concat(messagesByTeam.all);
                }
                messages.map(function (message) {
                    if (message) {
                        return message.content;
                    } else {
                        return "";
                    }
                });

                // drop end events
                messages = messages.filter(function (message) {
                    return message.event !== "end";
                });

                var eventsMessage = Messages.mkEventsMessage(player, players, bots, config, roundCounter, messages);

                sendToPlayerTyped(player, "eventsMessage", eventsMessage);
            });

            if (!round.world.finished) {
                gameLoop(players, bots, asteroids, roundCounter + 1, statistics, config.loopTime, config.noWait);
            } else {
                finished = true;
                started = false;

                // Sending end message
                var winnerTeamId = _.chain(round.messages)
                    .filter(function (message) {
                        return message.content.event === "end";
                    })
                    .map(function (m) {
                        return m.content.winnerTeamId;
                    })
                    .first()
                    .value();

                var playerPromises = players.map(function (player) {
                    return sendToPlayerTyped(player, "endMessage", Messages.mkEndMessage(player, bots, winnerTeamId));
                });
                var spectatorPromises = sendToSpectatorsTyped("endSummary", Messages.mkEndSummaryMessage(players, bots, winnerTeamId));

                var promises = playerPromises.concat(spectatorPromises);

                writeGameLog();

                // TODO: Handle this properly
                if (true || !keepAlive) {
                    Bluebird.all(promises).then(function () {
                        sockets.forEach(function (socket) {
                            socket.terminate();
                        });
                        onEndCallback();
                    });
                }
            }
        }
    }

    return {
        start: start
    };
}

module.exports = Game;
