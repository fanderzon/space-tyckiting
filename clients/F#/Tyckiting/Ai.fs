module Ai

module DummyAi = 
    open Domain
    open Actions
    open CommonLibrary
    
    let rnd = System.Random()
    let rand() = rnd.Next(-2, 2)
    
    let makeDecisions (round : Round) : Action list = 
        let movePos curr = 
            match curr + rand() with
            | x when x > 5 -> curr - 1
            | x when x < -5 -> curr + 1
            | x -> x
        
        let moveBot (bot : Bot) = 
            move bot.botId { x = movePos bot.pos.x
                             y = movePos bot.pos.y }
        
        let radarBot (bot : Bot) = 
            radar bot.botId { x = bot.pos.x + rand()
                              y = bot.pos.y + rand() }
        
        let cannonBot (bot : Bot) = 
            cannon bot.botId { x = bot.pos.x + rand()
                               y = bot.pos.y + rand() }
        
        let liveBots = round.state.you.bots |> List.filter (fun {alive = a} -> a)
        let moveActions = liveBots |> List.map moveBot
        
        let radarActions = 
            liveBots
            |> nth 0
            |> Option.map radarBot
            |> Option.toList
        
        let cannonAction = liveBots |> List.map cannonBot
        moveActions @ radarActions @ cannonAction

// Add your Ai here
module AiPicker = 
    open Domain
    
    let getAi = function 
        | Dummy -> DummyAi.makeDecisions
// Add mapping to your Ai here
