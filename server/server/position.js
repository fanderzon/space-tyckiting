"use strict";

var typify = require("./typify.js");

function make(x, y) {
    typify.assert("number", x);
    typify.assert("number", y);

    return {
        x: x,
        y: y
    };
}

// Max-distance
function distance(a, b) {
    typify.assert("position", a);
    typify.assert("position", b);

    var dx = Math.abs(a.x - b.x);
    var dy = Math.abs(a.y - b.y);
    var dz = Math.abs(a.x + a.y - b.x - b.y);

    return Math.max(dx, dy, dz);
}

function eq(a, b) {
    typify.assert("position", a);
    typify.assert("position", b);

    return a.x === b.x && a.y === b.y;
}

function roundCubeToNearestHex(fc) {
    // typify.assert("floatcube", fc);
    // Simply rounding would work in most of the cases.
    // The extra logic is there to handle few special cases,
    // where a position would round to wrong hex and possibly out of area.
    // The logic is explained here:
    // http://www.redblobgames.com/grids/hexagons/#rounding
    var rx = Math.round(fc.x);
    var ry = Math.round(fc.y);
    var rz = Math.round(fc.z);

    var xDiff = Math.abs(rx - fc.x);
    var yDiff = Math.abs(ry - fc.y);
    var zDiff = Math.abs(rz - fc.z);

    if (xDiff > yDiff && xDiff > zDiff) {
        rx = -ry - rz;
    } else if (yDiff > zDiff) {
        ry = -rx - rz;
    } else {
        rz = -rx - ry;
    }
    return {x: rx, y: rz};
}

function clampPosition(pos, radius) {
    var d = distance({x: 0, y: 0}, pos);

    if (d === 0 || d <= radius) {
        // When in radius, we can simply return the position
        return pos;
    } else {
        // We clip the position to nearest in radius
        var t = 1.0 * radius / d;

        var cx = pos.x * t;
        var cy = (-pos.x - pos.y) * t;
        var cz = pos.y * t;

        // We need to round the floating point location to nearest hex
        return roundCubeToNearestHex({x: cx, y: cy, z: cz});
    }
}

// clamp
function clamp(pos, fieldRadius) {
    typify.assert("position", pos);
    typify.assert("nat", fieldRadius);

    return clampPosition(pos, fieldRadius);
}

var origo = make(0, 0);

function neighbours(pos, radius) {
    typify.assert("position", pos);
    typify.assert("nat", radius);

    var result = [];

    for (var x = pos.x - radius; x <= pos.x + radius; x++) {
        for (var y = pos.y - radius; y <= pos.y + radius; y++) {
            var newPos = make(x, y);
            if (distance(pos, newPos) <= radius && !eq(pos, newPos)) {
                result.push(newPos);
            }
        }
    }

    return result;
}

module.exports = {
    make: make,
    distance: distance,
    eq: eq,
    clamp: clamp,
    origo: origo,
    neighbours: neighbours
};
