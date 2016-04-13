//
//  ai.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 04/04/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation

// Dummy bot. Moves randomly around the board.
struct DummyAI: BaseAI {
    let teamId: Int
    let config: Config

    func makeDecisions(bots  bots: [Bot], events: [Event]) -> [Action] {
        return bots.filter { $0.isAlive }
                   .map { return self.moveBot($0) }
    }

    func moveBot(bot: Bot) -> Action {
        return Action(type: .Move, botId: bot.botId, position: randomPosition(bot))
    }

    func randomPosition(bot: Bot) -> Position {
        if let pos = bot.pos {
            let minX = max(-config.fieldRadius, pos.x - config.move)
            let maxX = min( config.fieldRadius, pos.x + config.move)
            let minY = max(-config.fieldRadius, pos.y - config.move)
            let maxY = min( config.fieldRadius, pos.y + config.move)

            return Position(x: RandomInt(minX...maxX), y: RandomInt(minY...maxY))
        } else {
            return Position(x: 0, y: 0)
        }
    }
}