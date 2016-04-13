/*global window*/
"use strict";

require.config({
    paths: {
        "jquery": "libs/jquery.min",
        "lodash": "libs/lodash.min",
        "promise": "libs/bluebird.min",
        "svg": "libs/svg.min"
    }
});

if (!window.requireTestMode) {
    // The server provides the bot to start
    require(["main"], function () { });
}
