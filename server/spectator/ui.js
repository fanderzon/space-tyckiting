/*global define*/
"use strict";

define([
    "jquery",
    "lodash",
    "colors"],
    function ($, _, Colors) {

        var DESTROYED_COLOR = "#777";

        return function () {
            var statusTemplate =
                "<li class=\"status\" data-id=\"<%= id %>\">" +
                "   <span class=\"idbox\" style=\"border: 1px solid <%=color%>; background-color: <%=fill%>\"></span>" +
                "   <span class=\"name\"><%= name %></span><span>, </span>" +
                "   <span class=\"hp\"><%= hp %></span> / " +
                "   <span class=\"max\"><%= max %></span>" +
                "</li>";

            var $statusArea = $("#team .statuses");

            function addBot(bot, teamIndex, config) {
                if ($statusArea.find("[data-id=\"" + bot.botId + "\"]").length === 0) {
                    $statusArea.append(
                        _.template(statusTemplate, {
                            color: Colors.main(teamIndex),
                            fill: Colors.bg(teamIndex),
                            id: bot.botId,
                            name: bot.name,
                            hp: bot.hp,
                            max: config.startHp
                        })
                    );
                }
            }

            function updateBot(bot) {
                var $status = $statusArea.find(".status[data-id=\"" + bot.botId + "\"]");
                if ($status.length > 0) {
                    $status.find(".hp").html(bot.hp > 0 ? bot.hp : 0);
                    if (bot.hp <= 0) {
                        $status.find(".idbox").css("background-color", DESTROYED_COLOR);
                    }
                }
            }

            function reset() {
                $statusArea.html("");
            }

            function hasBot(id) {
                return $statusArea.find("[data-id=\"" + id + "\"]").length > 0;
            }

            return {
                addBot: addBot,
                updateBot: updateBot,
                hasBot: hasBot,
                reset: reset
            };
        };
    });
