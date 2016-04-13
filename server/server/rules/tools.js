"use strict";

var _ = require("lodash");

function filterActionsByType(actions, type) {
    return _.where(actions, {type: type});
}

function createMessage(target, content) {
    return {
        target: target,
        content: content
    };
}

function botInfo(bot) {
    return botInfoImpl(bot, false, false);
}

function botInfoWithPosition(bot) {
    return botInfoImpl(bot, true, false);
}

function botInfoWithPositionAndHp(bot) {
    return botInfoImpl(bot, true, true);
}

function botInfoImpl(bot, includePosition, includeHp) {
    var info = {
        botId: bot.botId,
        name: bot.name,
        teamId: bot.teamId,
        alive: bot.hp > 0 // exactly 0 hp is dead.
    };

    if (includePosition) {
        info.pos = bot.pos;
    }

    if (includeHp) {
        info.hp = bot.hp;
    }
    return info;
}

module.exports = {
    filterActionsByType: filterActionsByType,
    botInfoWithPosition: botInfoWithPosition,
    botInfoWithPositionAndHp: botInfoWithPositionAndHp,
    botInfo: botInfo,
    createMessage: createMessage
};
