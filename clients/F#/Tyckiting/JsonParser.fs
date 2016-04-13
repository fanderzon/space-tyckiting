module JsonParser

open FSharp.Data
open Domain
open CommonLibrary
open FSharp.Data.JsonExtensions

module private JsonObjectParser =
    let asInteger (jsonValue : JsonValue) = 
        jsonValue.AsInteger()

    let asString (jsonValue : JsonValue) = 
        jsonValue.AsString()

    let asBool (jsonValue : JsonValue) = 
        jsonValue.AsBoolean()

    let asArray (jsonValue : JsonValue) = 
        jsonValue.AsArray()

    let getProperty (jsonValue : JsonValue) name selector = 
        jsonValue.TryGetProperty(name)  |> Option.map selector

    let combine (f : unit -> Option<_>) (s : unit -> Option<_>) =
        f() |> Option.bind (fun x -> s() |> Option.map (fun y -> x,y))

    let combinef (f : unit -> Option<_>) (s : unit -> Option<_>) selector  =
        fun () -> f() |> Option.bind (fun x -> s() |> Option.map (fun y -> selector x y))

    let combinef2 (f : unit -> Option<_>) (s : unit -> Option<_>) =
        let tup x y = x,y 
        combinef f s tup

    let combinef3 (f : unit -> Option<_>) (s : unit -> Option<_>) =
        let tup x (y,z) = x,y,z
        combinef f s tup

    let combinef4 (f : unit -> Option<_>) (s : unit -> Option<_>) =
        let tup x (y,z,a) = x,y,z,a
        combinef f s tup

    let combinef5 (f : unit -> Option<_>) (s : unit -> Option<_>) =
        let tup x (y,z,a,b) = x,y,z,a,b
        combinef f s tup

    let combinef6 (f : unit -> Option<_>) (s : unit -> Option<_>) =
        let tup x (y,z,a,b,c) = x,y,z,a,b,c
        combinef f s tup

    let getPostition (jsonValue : JsonValue) =
        let tryGetPosition name selector = getProperty jsonValue name selector

        let getX() = tryGetPosition "x" asInteger
        let getY() = tryGetPosition "y" asInteger

        getY |> combine getX
             |> Option.map (fun (xPos,yPos) -> {x = xPos; y = yPos} )

    let getSingleEvent (jsonValue : JsonValue) = 
        let getAsInteger name = jsonValue.TryGetProperty(name) |> Option.map asInteger 
        let getEventPosition() = jsonValue.TryGetProperty("pos") |> Option.bind getPostition

        let getEvent name selector = getProperty jsonValue name selector
        let getEventName() = getEvent "event"  asString
        let getBotId() = getAsInteger "botId" |> Option.map BotId

        let getAction = function
            | "noaction" -> 
                getBotId() |> Option.map NoAction 
            | "move" -> 
                getBotId |> combine getEventPosition 
                         |> Option.map (fun (pos, botId) ->  Event.Move (botId,pos))
            | "damaged" ->  
                getBotId |> combine (fun () -> getAsInteger "damage")
                         |> Option.map (fun (d, botId) -> Damaged (botId, d) )
            | "die" -> 
                getBotId() |> Option.map Die 
            | "see" -> 
                getBotId |> combine getEventPosition 
                         |> Option.bind (fun (pos, botId) -> getAsInteger "source" |> Option.map (fun s -> pos,botId,s) )
                                   |> Option.map (fun (pos,botId,source) -> See (botId, BotId source, pos))
            | "hit" -> 
                getBotId |> combine (fun () -> getAsInteger "source") 
                         |> Option.map (fun (source, botId) -> Hit (botId, BotId source) )
            | "radarEcho" -> 
                getEventPosition() |> Option.map RadarEcho
            | "detected" -> 
                getBotId() |> Option.map Detected 
            | "seeAsteroid" -> 
                getEventPosition() |> Option.map SeeAsteroid
            | _ -> None

        

        getEventName() |> Option.bind getAction


    let getBot (jsonValue : JsonValue) =
        let tryGetBot name selector = getProperty jsonValue name selector

        let getBotId() = tryGetBot "botId" asInteger |> Option.map BotId
        let getName() = tryGetBot "name" asString 
        let getAlive() = tryGetBot "alive" asBool 
        let getHp() = tryGetBot "hp" asInteger
        let getTeamId() = tryGetBot "teamId" asInteger |> Option.map TeamId
        let getPos() = jsonValue.TryGetProperty("pos") |> Option.bind getPostition 

        let createBot = function
            | tId,bId,n,a,h,p ->  { botId = bId; name = n; alive = a; hp = h; teamId = tId; pos = p}

        getPos |> combinef2 getHp
               |> combinef3 getAlive
               |> combinef4 getName
               |> combinef5 getBotId
               |> combinef6 getTeamId
               |> (fun f -> f())
               |>  Option.map createBot


    let getTeam (jsonValue : JsonValue) = 
        let tryGetEvent name selector = getProperty jsonValue name selector

        let getTeamId() = tryGetEvent "teamId" asInteger |> Option.map TeamId
        let getName() = tryGetEvent "name" asString
        let getBots() = tryGetEvent "bots" asArray |> Option.map ((Array.map getBot) >> ( Array.choose id) >> Array.toList)

        getTeamId |> combinef2 getName
                  |> combinef3 getBots
                  |> (fun f -> f())
                  |> Option.map (fun (b,n,id) ->  { name = n; teamId = id; bots = b})





module JsonParserProvider =

    [<Literal>]
    let join = """
    {
      "type": "join",
      "teamName": "I'm having a problem with this."
    }
    """
    type JoinFormat = JsonProvider<join, RootName="root">
    let getJoinMessage teamName =
        let (TeamNameParameter name) = teamName
        let newIssue = JoinFormat.Root("join", name)
        newIssue.JsonValue.ToString( JsonSaveOptions.DisableFormatting )



    [<Literal>]
    let action = """
    {
      "type": "someaction",
      "botId":3,
      "pos":{"x":0,"y":0}
    }
    """
    type ActionFormat = JsonProvider<action, RootName="root">
    let getActionMessage botAction =
        let getActionString = function
            | Move (BotId id, pos) -> "move", id, pos
            | Radar (BotId id, pos)  -> "radar", id, pos
            | Cannon (BotId id, pos)  -> "cannon", id, pos

        botAction |> getActionString 
                  |> fun (action, botId, pos) -> ActionFormat.Root(action, botId, ActionFormat.Pos(pos.x,pos.y))
                  |> fun json -> json.JsonValue


    [<Literal>]
    let actions = """
    {
      "type": "actions",
      "roundId": "1",
      "actions":[]
    }
    """
    type ActionsFormat = JsonProvider<actions, RootName="root">
    let getActionsMessage (roundId, (botActions: Action list)) =
        ActionsFormat.Root("actions", roundId, ( botActions |> List.map getActionMessage |> List.toArray ) ) |> fun json -> json.JsonValue.ToString()

    [<Literal>]
    let configJson = """
    {
        "bots":3,
        "fieldRadius":14,
        "move":2,
        "startHp":10,
        "cannon":1,
        "radar":3,
        "see":2,
        "maxCount":200,
        "loopTime":300
    }
    """



    type ConfigFormat = JsonProvider<configJson, RootName="root">


    let getConfig json =

        tryResult( fun () -> ConfigFormat.Parse(json)) |> map (fun simple -> { bots = simple.Bots; 
                                                                               fieldRadius = simple.FieldRadius;
                                                                               move = simple.Move; 
                                                                               startHp = simple.StartHp; 
                                                                               cannon = simple.Cannon; 
                                                                               radar = simple.Radar; 
                                                                               see = simple.See; 
                                                                               maxCount = simple.MaxCount; 
                                                                               loopTime = simple.LoopTime; })
    


    [<Literal>]
    let connected = """
    {
        "config": { }
    }
    """

    type ConnectedFormat = JsonProvider<connected>
    let getConnectedMessage json =
        tryResult( fun () -> ConnectedFormat.Parse(json)) |> map (fun j -> j.Config.JsonValue.ToString())
                                                          |> bind getConfig

    let getGameState (jsonValue : JsonValue) = 
        let getTeamFromJson (j : JsonValue) = 
            JsonObjectParser.getTeam j |> toResult (JsonParseError (j.ToString()))
        let yourTeam = jsonValue.TryGetProperty("you") |> toResult (JsonParseError "Missing your team in json") 
                                                       |> bind getTeamFromJson
                                                       
        let otherTeams = seq {  for b in (jsonValue?otherTeams).AsArray() do yield b } |> Seq.map getTeamFromJson
                                                                                  |> Seq.choose toOption
                                                                                  |> Seq.toList

        jsonValue?config.ToString() |> getConfig
                                    |> bind (fun config -> yourTeam |> map (fun you -> (config,you)))
                                    |> map (fun (config,you) -> {config = config; 
                                                                        you = you; 
                                                                        otherTeams = otherTeams;})
    [<Literal>]
    let startType = """
    {
        "you": { },
        "config":{ },
        "otherTeams":[ ]
    }

    """



    type StartProvider = JsonProvider<startType>

    let getStartMessage json =
        tryOption (fun () ->  StartProvider.Parse(json).JsonValue) |> toResult (JsonParseError json)
                                                                   |> bind getGameState
        
        
    [<Literal>]
    let eventsType = """
    {
        "roundId": 0,
        "you": {},
        "config":{},
        "otherTeams": [],
        "events":[]
    }

    """

    type EventsProvider = JsonProvider<eventsType>



    let getEventsMessage json =
        let events = EventsProvider.Parse(json)
    
        let parsedEvents = seq {  for b in events.Events do  yield b.JsonValue } 
                                      |> Seq.toList
                                      |> List.map JsonObjectParser.getSingleEvent 
                                      |> List.choose id

        events.JsonValue |> getGameState
                         |> map (fun gameState -> {id = events.RoundId; 
                                                   state = gameState;
                                                   events = parsedEvents })


    [<Literal>]
    let endType = """
    {
        "type": "end",
        "you": {},
        "winnerTeamId":1
    }

    """

    type EndFormat = JsonProvider<endType>


    let getEndMessage json =
        let endJson = EndFormat.Parse(json)

        let getWinnerTeamId = 
            try
                endJson.JsonValue.TryGetProperty("winnerTeamId") |> Option.map JsonObjectParser.asInteger
            with _  -> None
        
        let getTeamFromJson (jsonValue : JsonValue) = 
            JsonObjectParser.getTeam jsonValue |> toResult (JsonParseError (endJson.You.JsonValue.ToString()))
    
        getTeamFromJson endJson.You.JsonValue  |> map (fun team ->  team, getWinnerTeamId |> Option.map TeamId )




    [<Literal>]
    let jsonType = """
    {
        "type":"connected"
    }
    """

    type JsonTypeProvider = JsonProvider<jsonType>
    let parse json = 
        let matching = function
            | "connected" -> json |> getConnectedMessage |> map Connected 
            | "start" ->  json |> getStartMessage |> map Started 
            | "events" -> json |> getEventsMessage |> map Events 
            | "end" -> json |> getEndMessage |> map End 

            | _ -> Failure (JsonParseError (sprintf "Couldn't match type: %A" json))

        tryResult (fun () -> JsonTypeProvider.Parse(json).Type) |> bind matching 
