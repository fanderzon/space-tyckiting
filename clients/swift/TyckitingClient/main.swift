//
//  main.swift
//  TyckitingClient
//
//  Created by Markus Kauppila on 21/03/15.
//  Copyright (c) 2015 Futurice. All rights reserved.
//

import Foundation
import CommandLine

let hostOption = StringOption(shortFlag: "H", longFlag: "host", required: true, helpMessage: "Host to connect to. Defaul: localhost", defaultValue: "localhost")
let portOption = IntOption(shortFlag: "P", longFlag: "port", required: false, helpMessage: "Port to connect to. Default: 3000", defaultValue: 3000)
let nameOption = StringOption(shortFlag: "nm", longFlag: "name", required: false, helpMessage: "Bot's name. Default: bot", defaultValue: "bot")
let aiOption = StringOption(shortFlag: "a", longFlag: "ai", required: false, helpMessage: "Ai packaage. Default: dummy", defaultValue: "dummy")

let options = CommandLine()
options.addOptions(
    hostOption,
    portOption,
    nameOption,
    aiOption
)

let (success, error) = options.parse()
if success {
    let client = Client(host: hostOption.value,
                        port: portOption.value,
                        name: nameOption.value,
                          ai: aiOption.value)
    client.start()
    NSRunLoop.currentRunLoop().run()
} else {
    print(error!)
    options.printUsage()
}

