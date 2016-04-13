/*global define*/
"use strict";

var MAIN_COLORS = [
    "#00B2ED",
    "#F0A800"
];

var BG_COLORS = [
    "#007CA6",
    "#B37D00"
];

function main(index) {
    return MAIN_COLORS[index % MAIN_COLORS.length];
}

function bg(index) {
    return BG_COLORS[index % BG_COLORS.length];
}

var Colors = {
    main: main,
    bg: bg
};
