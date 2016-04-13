/*global define*/
"use strict";

define([], function () {

    return function Hex(q, r) {
        this.q = q;
        this.r = r;
        this.toString = function () {
            return "{ q: " + q + ", r: " + r + " }";
        };
    };
});
