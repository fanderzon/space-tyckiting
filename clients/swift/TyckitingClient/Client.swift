//
//  Client.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 05/04/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation

import Argo

class Client: NSObject, SRWebSocketDelegate {

    let socket: SRWebSocket
    let hostURL: NSURL
    let name: String

    var ai: BaseAI?

    init(host: String, port: Int, name: String, ai: String) {
        self.hostURL = NSURL(string: "ws://\(host):\(port)")!
        self.socket = SRWebSocket(URLRequest: NSURLRequest(URL: self.hostURL))
        self.name = name

        super.init()

        socket.delegate = self
    }

    func createAI(name  name: String, message: ConnectedMessage) -> BaseAI {
        switch name {
        case "dummy":
            fallthrough
        default:
            return DummyAI(teamId: message.teamId, config: message.config)
        }
    }

    // MARK: Client Events
    func onConnected(message: ConnectedMessage) {
        ai = createAI(name: self.name, message: message)

        print("onConnected: \(message)")
        sendMessage([
            "type": "join",
            "teamName": self.name
            ])
    }

    func onStart(message: StartMessage) {
        print("onStart: \(message)")
    }

    func onEvent(message: EventsMessage) {
        print("Round \(message.roundId)")
        let responses = ai!.makeDecisions(bots: message.you.bots, events: message.events)

        sendMessage([
            "type": "actions",
            "roundId": message.roundId,
            "actions": responses.map { $0.toDictionary() }
            ])
    }

    func onEnd(message: EndMessage) {
        if let winnerTeamId = message.winnerTeamId {
            if winnerTeamId == message.you.teamId {
                print("Game ended: you win!")
            } else {
                print("Game ended: you lose!")
            }
        } else {
            print("Game ended: it's a draw")
        }

        socket.close()
    }

    func onError(message: ErrorMessage) {
        print(message)
    }

    // MARK:

    func sendMessage(message: [String: AnyObject]) {
        do {
            let data = try NSJSONSerialization.dataWithJSONObject(message, options: NSJSONWritingOptions())
            let debugMessage = NSString(data: data, encoding: NSUTF8StringEncoding)
            print("Sending message: \(debugMessage)")
            socket.send(data)
        } catch {
            print("Failed to send message: \(message)")
        }
    }

    func parseAndHandleMessage(message: AnyObject) {
        if let messageString = message as? String,
           let rawJson: AnyObject = parseJson(messageString) {
                let type = messageType(message)
                switch type {
                case "connected":
                    if let connectedMessage: ConnectedMessage = decode(rawJson) {
                        onConnected(connectedMessage)
                    }
                case "start":
                    if let startMessage: StartMessage = decode(rawJson) {
                        onStart(startMessage)
                    }
                case "events":
                    if let eventsMessage: EventsMessage = decode(rawJson) {
                        onEvent(eventsMessage)
                    }
                case "end":
                    if let endMessage: EndMessage = decode(rawJson) {
                        onEnd(endMessage)
                    }
                case "error":
                    if let errorMessage: ErrorMessage = decode(rawJson) {
                        onError(errorMessage)
                    }
                default:
                    print("Unknown message type: \(type)")
                }
        }
    }

    func messageType(message: AnyObject) -> String {
        if let jsonString = message as? String,
            let json = parseJson(jsonString),
            let type = json["type"] as? String {
                return type
        } else {
            return ""
        }
    }

    func parseJson(jsonString: NSString) -> Dictionary<String, AnyObject>? {
        if let data = jsonString.dataUsingEncoding(NSUTF8StringEncoding, allowLossyConversion: true),
            let json = try! NSJSONSerialization.JSONObjectWithData(data, options: NSJSONReadingOptions()) as? Dictionary<String, AnyObject> {
                return json
        } else {
            return nil
        }
    }

    // MARK: SRWebSocketDelegate

    func webSocket(webSocket: SRWebSocket!, didReceiveMessage message: AnyObject!) {
        parseAndHandleMessage(message)
    }

    func webSocketDidOpen(webSocket: SRWebSocket!) {
        print("WebSocket did open")
    }

    func webSocket(webSocket: SRWebSocket!, didFailWithError error: NSError!) {
        print("WebSocket connection error: \(error)")
    }

    func webSocket(webSocket: SRWebSocket!, didCloseWithCode code: Int, reason: String!, wasClean: Bool) {
        print("Websocket connection closed. Reason \(reason)")
        exit(0)
    }
    
    func start() {
        print("Connect to \(self.hostURL)")
        socket.open()
    }
}
