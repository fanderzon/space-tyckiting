//
//  BaseAI.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 05/04/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation

protocol BaseAI {
    init(teamId: Int, config: Config)

    /// Perform bot actions, based on events from last round.
    /// This is the only method that needs to be implemented in custom AIs.
    func makeDecisions(bots  bots: [Bot], events: [Event]) -> [Action]
}
