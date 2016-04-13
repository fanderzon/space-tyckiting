//
//  messages.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 02/04/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation

import Argo
import Curry

public struct ConnectedMessage {
    let type: String
    let teamId: Int
    let config: Config
}

extension ConnectedMessage: Decodable {
    static public func decode(j: JSON) -> Decoded<ConnectedMessage> {
        return curry(ConnectedMessage.init)
            <^> j <| "type"
            <*> j <| "teamId"
            <*> j <| "config"
    }
}

public struct StartMessage {
    let type: String
    let you: Team
    let config: Config
    let otherTeams: [Team]
}

extension StartMessage: Decodable {
    static public func decode(j: JSON) -> Decoded<StartMessage> {
        return curry(StartMessage.init)
            <^> j <| "type"
            <*> j <| "you"
            <*> j <| "config"
            <*> j <|| "otherTeams"
    }
}

public struct EventsMessage {
    let type: String
    let roundId: Int
    let config: Config
    let you: Team
    let otherTeams: [Team]
    let events: [Event]
}

extension EventsMessage: Decodable {
    static public func decode(j: JSON) -> Decoded<EventsMessage> {
        return curry(EventsMessage.init)
            <^> j <| "type"
            <*> j <| "roundId"
            <*> j <| "config"
            <*> j <| "you"
            <*> j <|| ["otherTeams"]
            <*> j <|| ["events"]
    }
}

public struct EndMessage {
    let type: String
    let you: Team
    let winnerTeamId: Int?
}

extension EndMessage: Decodable {
    static public func decode(j: JSON) -> Decoded<EndMessage> {
        return curry(EndMessage.init)
            <^> j <| "type"
            <*> j <| "you"
            <*> j <|? "winnerTeamId"
    }
}

public struct ErrorMessage {
    let type: String
}

extension ErrorMessage: Decodable {
    static public func decode(j: JSON) -> Decoded<ErrorMessage> {
        return curry(ErrorMessage.init)
            <^> j <| "type"
    }
}

