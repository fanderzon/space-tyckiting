/*global define, WebSocket, window*/
"use strict";

define([
    "jquery",
    "promise",
    "messageBox",
    "mainScreen",
    "ui"],
    function ($,
             Promise,
             MessageBox,
             MainScreen,
             Ui) {

        var TARGET_URL = "ws://" + window.location.host;

        var HEX_SIZE = 18;

        var messageBox = new MessageBox();

        var ui = new Ui();

        var grid = null;

        var socket = new WebSocket(TARGET_URL);

        socket.onopen = function () {
            console.info("connected");

            var config;

            socket.onclose = function () {
                console.info("close");
            };

            socket.onmessage = function (rawContent) {
                var content = JSON.parse(rawContent.data);

                console.info("message", content);

                if (content.type === "connected") {

                    config = content.config;
                    console.log('config', config);

                    grid = grid || new MainScreen("mainScreen", HEX_SIZE, config.fieldRadius, config.cannon, config.radar);

                    socket.send(JSON.stringify({type: "spectate", data: {}}));

                } else if (content.type === "start") {
                    clearMessages();
                    grid.clearAll();
                    ui.reset();
                } else if (content.type === "round") {

                    grid.clear();

                    var actions = content.actions.reduce(function (memo, action) {
                        memo[action.botId] = action;
                        return memo;
                    }, {});

                    content.asteroids.forEach(function (asteroid) {
                        grid.addAsteroid(asteroid);
                    });

                    content.teams.forEach(function (team, teamIndex) {
                        team.bots.forEach(function (bot) {
                            if (!grid.hasShip(bot.botId)) {
                                grid.addShip(bot.botId, bot.pos.x, bot.pos.y, teamIndex);
                            }
                            if (bot.hp <= 0) {
                                grid.destroyShip(bot.botId);
                            } else {
                                grid.moveShip(bot.botId, bot.pos.x, bot.pos.y);
                            }

                            var action = actions[bot.botId];

                            if (action) {
                                if (action.type === "radar") {
                                    grid.radar(bot.botId, action.pos.x, action.pos.y, teamIndex);
                                } else if (action.type === "cannon") {
                                    grid.blast(bot.botId, action.pos.x, action.pos.y, teamIndex);
                                }
                            }

                            if (!ui.hasBot(bot.botId)) {
                                ui.addBot(bot, teamIndex, config);
                            } else {
                                ui.updateBot(bot);
                            }
                        });
                    });
                } else if (content.type === "end") {
                    showMessage("Server", "server", "Game ended. " + content.winner + " won!", true);
                }
            };
        };

        function clearMessages() {
            messageBox.clear();
        }

        function showMessage(source, id, message, friend) {
            messageBox.addMessage(source, id, message, friend ? "friend" : "foe");
        }
    });
