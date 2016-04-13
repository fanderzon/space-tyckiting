/*global define*/
"use strict";

define([], function () {

    return function Cube(x, y, z) {
        this.x = x;
        this.y = y;
        this.z = z;

        this.toString = function () {
            return "{ x: " + this.x + ", y: " + this.y + "z: " + this.z + " }";
        };
    };
});
