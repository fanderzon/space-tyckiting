/*global define*/
"use strict";

define([
    "lodash",
    "hex",
    "cube",
    "point"
], function (_,
     Hex,
     Cube,
     Point) {

    var SQRT_3 = Math.sqrt(3);

    function hexToPoint(hex, size) {
        var x = size * SQRT_3 * (hex.q + hex.r / 2);
        var y = size * 1.5 * hex.r;

        return new Point(x, y);
    }

    function addHex(a, b) {
        return new Hex(a.q + b.q, a.r + b.r);
    }

    function hexToCube(hex) {
        return new Cube(hex.q, -(hex.q + hex.r), hex.r);
    }

    function cubeToHex(cube) {
        return new Hex(cube.x, cube.z);
    }

    function addCube(a, b) {
        return new Cube(a.x + b.x, a.y + b.y, a.z + b.z);
    }

    function hexDistance(a, b) {
        var dx = Math.abs(a.q - b.q);
        var dy = Math.abs(-a.q - a.r + b.q + b.r);
        var dz = Math.abs(a.r - b.r);
        return Math.max(dx, dy, dz);
    }

    function hexesInRadius(center, radius) {
        var results = [];
        var dx;
        var dz;
        var dy;

        var rowStart;
        var rowEnd;

        // Creates hexagonal hex grid. Rather optimized version.
        for (dx = -radius; dx <= radius; ++dx) {
            rowStart = Math.max(-radius, -dx - radius);
            rowEnd = Math.min(radius, -dx + radius);
            for (dy = rowStart; dy <= rowEnd; ++dy) {
                dz = -dx - dy;
                results.push(addHex(center, new Hex(dx, dz)));
            }
        }
        return results;
    }

    return {
        hexDistance: hexDistance,
        addHex: addHex,
        addCube: addCube,
        hexToCube: hexToCube,
        hexToPoint: hexToPoint,
        cubeToHex: cubeToHex,
        hexesInRadius: hexesInRadius
    };
});
