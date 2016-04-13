#r "../packages/FSharp.Data.2.2.0/lib/net40/FSharp.Data.dll"
#load "Domain.fs"
#load "CommonLibrary.fs"
#load "Network.fs"
#load "JsonParser.fs"
#load "Actions.fs"
#load "Ai.fs"
#load "ParameterParser.fs"
#load "Main.fs"

open FSharp.Data
open CommonLibrary
open Network
open System
open JsonParser.JsonParserProvider
open Domain
open Actions
open Ai
open Main



main [||] 
