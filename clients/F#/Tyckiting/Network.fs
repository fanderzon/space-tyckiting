module Network

open System.Net.WebSockets
open System.Threading
open System.Text
open System
open CommonLibrary
open Domain

let getHostUri { host = HostParameter h; port = PortParameter p } = 
    tryResult (fun () -> new Uri(sprintf "ws://%s:%d" h p))

let private awaitTask = Async.AwaitIAsyncResult >> Async.Ignore

let connect uri = 
    let connectedSocket = 
        let socket = new ClientWebSocket()
        socket.ConnectAsync(uri, CancellationToken.None) |> (awaitTask >> Async.RunSynchronously)
        socket

    let rec waitWhileConnecting (clientSocket: ClientWebSocket) =
        match clientSocket with
        | socket when socket.State = WebSocketState.Connecting -> waitWhileConnecting socket
        | socket when socket.State = WebSocketState.Open -> Success socket
        | socket -> fail (ConnectionError socket.State)
        
    waitWhileConnecting connectedSocket

let send (socket : ClientWebSocket) (json : string) = 
    let encoded = tryResult (fun () -> (new UTF8Encoding()).GetBytes(json))
    let sendAsync data = 
        tryResult (fun () -> socket.SendAsync(data, WebSocketMessageType.Text, true, CancellationToken.None))
    encoded
    |> map (fun bytes -> new ArraySegment<byte>(bytes))
    |> bind sendAsync
    |> map (awaitTask >> Async.RunSynchronously)

let receive (socket : ClientWebSocket) = 
    let receiveAsync = 
        let decode data = 
            let length = 
                match Seq.tryFindIndex (fun b -> b = byte (0)) data with
                | Some i -> i
                | None -> Seq.length data
            tryResult (fun () -> (new UTF8Encoding()).GetString(data, 0, length))
        async { 
            let buffer = Array.init 2000 (fun _ -> byte (0))
            try 
                do! awaitTask (socket.ReceiveAsync(new ArraySegment<byte>(buffer), CancellationToken.None))
                return decode buffer
            with ex -> return fail (ExceptionError ex)
        }
    receiveAsync |> Async.RunSynchronously