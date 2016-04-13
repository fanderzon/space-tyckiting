// Learn more about F# at http://fsharp.net
// See the 'F# Tutorial' project for more help.

open Main
open System

[<EntryPoint>]
let main argv = 
    let result = main argv
    Console.ReadKey() |> ignore
    result