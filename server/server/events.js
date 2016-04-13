"use strict";

function winningEvent(winningTeamId) {
    return {
        event: "end",
        winnerTeamId: winningTeamId
    };
}

function tieEvent() {
    return {
        event: "end",
        winnerTeamId: -1
    };
}

module.exports = {
    winningEvent: winningEvent,
    tieEvent: tieEvent
};
