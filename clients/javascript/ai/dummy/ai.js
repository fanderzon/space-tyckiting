"use strict";

var _ = require("lodash");
var position = require("../../position.js");

// Change botNames and teamName to your choice.
var botNames = [
  "Zero",
  "One",
  "Two"
];

module.exports = function Ai() {
  function makeDecisions(roundId, events, bots, config) {
    bots.forEach(function(bot) {
      var ps = position.neighbours(position.make(bot.x, bot.y), config.move);
      var pos = ps[randInt(0, ps.length - 1)];
      bot.move(pos.x, pos.y);
    });

    _.each(events, function(event) {
      if (event.event === "noaction") {
        console.log("Bot did not respond in required time", event.data);
      }
    });
  }

  function randInt(min, max) {
    var range = max - min;
    var rand = Math.floor(Math.random() * (range + 1));
    return min + rand;
  }

  return {
    // The AI must return these three attributes
    botNames: botNames,
    makeDecisions: makeDecisions
  };
};
