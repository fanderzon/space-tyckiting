module Domain

open System.Net.WebSockets
open System

type Errors = 
    | ConnectionError of WebSocketState
    | JsonParseError of string
    | ExceptionError of Exception
    | ParameterError of string
    | Finish of string

type Ai = Dummy

type PortParameter = 
    | PortParameter of int

type HostParameter = 
    | HostParameter of string

type TeamNameParameter = 
    | TeamNameParameter of string

type CommandLineOptions = 
    { host : HostParameter
      port : PortParameter
      name : TeamNameParameter
      ai : Ai }

type Config = 
    { bots : int
      fieldRadius : int
      move : int
      startHp : int
      cannon : int
      radar : int
      see : int
      maxCount : int
      loopTime : int }

type Position = 
    { x : int
      y : int }

type BotId = 
    | BotId of int

type TeamId = 
    | TeamId of int

type Bot = 
    { botId : BotId
      name : string
      teamId : TeamId
      alive : bool
      pos : Position
      hp : int }

type Event = 
    | Damaged of BotId * int
    | Die of BotId
    | See of BotId * BotId * Position
    | RadarEcho of Position
    | SeeAsteroid of Position
    | Hit of BotId * BotId
    | Detected of BotId
    | Move of BotId * Position
    | Message of string
    | NoAction of BotId

type Team = 
    { name : string
      teamId : TeamId
      bots : Bot list }

type Action = 
    | Move of BotId * Position
    | Radar of BotId * Position
    | Cannon of BotId * Position

type GameState = 
    { config : Config
      you : Team
      otherTeams : Team list }

type Round = 
    { id : int
      state : GameState
      events : Event list }

type JsonType = 
    | Connected of Config
    | Started of GameState
    | Events of Round
    | End of Team * TeamId option
