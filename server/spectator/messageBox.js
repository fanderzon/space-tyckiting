/*global define*/
"use strict";

define(["jquery"], function ($) {

    return function () {

        var messages = {};

        var $box = $(".messageBox");

        function createLine(source, message, type) {
            var name = source.name;
            return $("<li class=\"message " + type + "\">[" + name + "] " + message + "</li>");
        }

        function addMessage(source, id, message, type) {
            if (!messages[id]) {
                $box.prepend(createLine(source, message, type));
                messages[id] = {source: source, message: message};
            }
        }

        function clear() {
            $box.find("li").remove();
        }

        return {
            clear: clear,
            addMessage: addMessage
        };
    };
});
