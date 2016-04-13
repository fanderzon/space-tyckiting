/* global it:true describe:true */
"use strict";

var radar = require("../../server/rules/radar.js");
var assert = require("assert");

describe("radar", function () {
    it("should return echoes for radars on the edge of the field", function () {
        var events = radar.events([{
            type: "radar",
            botId: 1,
            pos: {
                x: 5,
                y: 3
            }
        }], {
            bots: [
                {teamId: 1, botId: 1, pos: {x: 0, y: 0}},
                {teamId: 2, botId: 2, pos: {x: 5, y: 3}}
            ],
            asteroids: []
        }, {
            fieldRadius: 8,
            radar: 3
        });

        assert.equal(events.length, 1);
    });
});
