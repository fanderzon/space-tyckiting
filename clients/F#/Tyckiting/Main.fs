module Main

open Network
open CommonLibrary
open JsonParser.JsonParserProvider
open ParameterParser
open Domain
open Ai

let main argv = 
    let parametersResult = 
        argv
        |> List.ofArray
        |> parseCommandLine
        |> log
    
    let socketResult = parametersResult >>= getHostUri >>= connect
    
    let startGame socket (parameters : CommandLineOptions) = 
        let rec loop() = 
            let ai = AiPicker.getAi parameters.ai
            
            let decide = 
                function 
                | Events round -> getActionsMessage (round.id, ai round)
                | _ -> "{}"
            
            let checkStatus = 
                function 
                | Success(End({ teamId = myId }, Some winnerId)) -> 
                    if myId = winnerId then "You have won"
                    else "The other team has won"
                    |> Finish
                    |> fail
                | Success(End _) -> 
                    "It's a tie"
                    |> Finish
                    |> fail
                | x -> x
            
            let received = 
                socket
                |> receive
                |> logI "json:"
                >>= parse
                //          |> logI "received:"
                |> checkStatus
                |> map decide
                //           |> logI "sending:"
                >>= (send socket)
            
            match received with
            | Success _ -> loop()
            | Failure _ -> received
        
        let start name = 
            name
            |> getJoinMessage
            |> send socket
            >>= loop
        
        start parameters.name
    
    let startResult = 
        socketResult >>= (fun socket -> parametersResult >>= (fun parameters -> startGame socket parameters))
    match startResult with
    | Success _ -> ()
    | Failure f -> 
        match f with
        | ConnectionError e -> printfn "Connection error: %A" e
        | JsonParseError e -> printfn "Json parsing error: %A" e
        | ExceptionError e -> printfn "Exception error: %A" e
        | ParameterError e -> printfn "Parameter error: %A" e
        | Finish message -> printfn "%s" message
    0 // return an integer exit code
