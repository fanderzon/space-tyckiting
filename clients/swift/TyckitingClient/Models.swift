//
//  models.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 04/04/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation

import Argo
import Curry

enum ActionType: CustomStringConvertible {
    case Move
    case Radar
    case Cannon

    var description: String {
        switch self {
        case .Move: return "move";
        case .Radar: return "radar";
        case .Cannon: return "cannon";
        }
    }
}

struct Action {
    let type: ActionType
    let botId: Int
    let position: Position

    func toDictionary() -> [String: AnyObject] {
        return [
            "type": type.description,
            "botId": botId,
            "pos": [
                "x": position.x,
                "y": position.y
            ]
        ]
    }
}

public struct Config {
    let bots: Int
    let fieldRadius: Int
    let move: Int
    let startHp: Int
    let cannon: Int
    let radar: Int
    let see: Int
    let maxCount: Int
    let loopTime: Int
}

extension Config: Decodable {
    static func create(bots: Int)(fieldRadius: Int)(move: Int)(startHp: Int)(cannon: Int)(radar: Int)(see: Int)(maxCount: Int)(loopTime: Int) -> Config {
        return Config(bots: bots, fieldRadius: fieldRadius, move: move, startHp: startHp, cannon: cannon, radar: radar, see: see, maxCount: maxCount, loopTime: loopTime)
    }

    static public func decode(j: JSON) -> Decoded<Config> {
        // Can't be currified with `init` because Swift compilers balks
        // claiming that the expression is too complex
        return Config.create
            <^> j <| "bots"
            <*> j <| "fieldRadius"
            <*> j <| "move"
            <*> j <| "startHp"
            <*> j <| "cannon"
            <*> j <| "radar"
            <*> j <| "see"
            <*> j <| "maxCount"
            <*> j <| "loopTime"
    }
}

public struct Position {
    let x: Int;
    let y: Int;
}

extension Position: Decodable {
    static public func decode(j: JSON) -> Decoded<Position> {
        return curry(Position.init)
            <^> j <| "x"
            <*> j <| "y"
    }
}


public struct Bot {
    let botId: Int
    let name: String
    let teamId: Int
    let alive: Bool
    let pos: Position?
    let hp: Int?

    var isAlive: Bool {
        return alive
    }
}

extension Bot: Decodable {
    static public func decode(j: JSON) -> Decoded<Bot> {
        return curry(Bot.init)
            <^> j <| "botId"
            <*> j <| "name"
            <*> j <| "teamId"
            <*> j <| "alive"
            <*> j <|? "pos"
            <*> j <|? "hp"
    }
}


public struct Team {
    let name: String
    let teamId: Int
    let bots: [Bot]
}

extension Team: Decodable {
    static public func decode(j: JSON) -> Decoded<Team> {
        return curry(Team.init)
            <^> j <| "name"
            <*> j <| "teamId"
            <*> j <|| ["bots"]
    }
}

public struct Event {
    let event: String
    let botId: Int
    let source: Int?
    let damage: Int?
    let pos: Position?
    let winnerTeamId: Int?
}

extension Event: Decodable {
    static public func decode(j: JSON) -> Decoded<Event> {
        return curry(Event.init)
            <^> j <| "event"
            <*> j <| "botId"
            <*> j <|? "source"
            <*> j <|? "damage"
            <*> j <|? "pos"
            <*> j <|? "winnerTeamId"
    }
}
