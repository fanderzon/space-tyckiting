/* global describe:true */
"use strict";

var position = require("../server/position.js");
var jsc = require("jsverify");

describe("position", function () {
    jsc.property("eq is reflective", "integer", "integer", function (x, y) {
        var pos = position.make(x, y);
        return position.eq(pos, pos);
    });

    describe("clamp", function () {
        jsc.property("distance (clamp d pos) origo <= d", "nat", "integer", "integer", function (d, x, y) {
            var pos = position.make(x, y);
            var clamped = position.clamp(pos, d);
            return position.distance(clamped, position.origo) <= d;
        });

        jsc.property("clamp does nothing if position is closer to origo", "nat", "integer", "integer", function (d, x, y) {
            var pos = position.make(x, y);
            var clamped = position.clamp(pos, d);
            if (position.distance(pos, position.origo) <= d) {
                return position.eq(pos, clamped);
            } else {
                return position.distance(clamped, position.origo) === d;
            }
        });
    });

    describe("neighbours", function () {
        jsc.property("there are 6 neighbours at radius 1", "integer", "integer", function (x, y) {
            var pos = position.make(x, y);
            return position.neighbours(pos, 1).length === 6;
        });
    });
});
