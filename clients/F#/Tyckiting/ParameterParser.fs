module ParameterParser

open Domain
open CommonLibrary


let rec private parseCommandLineRec args optionsSoFar = 
        match args with 
        | [] -> 
            Success optionsSoFar  

        | "-H"::xs | "--host"::xs-> 
            match xs with 
                | [] -> Failure (ParameterError "Empty host")
                | x::xss -> parseCommandLineRec xss { optionsSoFar with host = HostParameter x} 

        | "-P"::xs | "--port"::xs-> 
            match xs with 
                | [] -> Failure (ParameterError "Empty port")
                | x::xss -> 
                            let port = match System.Int32.TryParse(x) with
                                            | true, value -> Success value
                                            | _ -> Failure (ParameterError ("Invalid port: " + x))
                            port |> map (fun p -> { optionsSoFar with port = PortParameter p})
                                 |> bind (parseCommandLineRec xss) 
        | "-n"::xs | "--name"::xs -> 
            match xs with 
                | [] -> Failure (ParameterError "Empty name")
                | x::xss -> parseCommandLineRec xss { optionsSoFar with name = TeamNameParameter x} 

        | "-a"::xs | "--ai"::xs -> 
            match xs with 
                | [] -> Failure (ParameterError "Empty ai")
                | x::xss ->  match x with
                                | "dummy" -> parseCommandLineRec xss { optionsSoFar with ai = Dummy} 
                                | x -> Failure (ParameterError ("Unrecognized ai: " + x))
        | "-h"::xs | "--help"::xs ->
            printf """ 
Available options:
    -h,--help                Show this help text
    -H,--host HOST           Host to connect to
    -P,--port PORT           Port to connect to
    -n,--name NAME           Bot's name
    -a,--ai AI               Select AI

                   """
            parseCommandLineRec xs optionsSoFar
        | x::xs -> 
            printfn "Option '%s' is unrecognized" x
            parseCommandLineRec xs optionsSoFar 

let parseCommandLine args = 
    let defaultOptions = {
            host = HostParameter "localhost";
            port = PortParameter 3000;
            name = TeamNameParameter "Winners";
            ai = Dummy
            }

    parseCommandLineRec args defaultOptions