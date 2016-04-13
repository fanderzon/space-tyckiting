module WebSock

using PyCall
@pyimport websocket

type WebSocketClient
  socket # WebSocket reference to python
  handler::Function
  debug::Bool
end

on_ws_error(ws, error) = error("WebSocket connection error $error")

# socket wrappers
function close(client::WebSocketClient)
  client.socket[:close]()
end

# callback for receiving message
function on_ws_message(client::WebSocketClient, raw_message)
  client.debug && info("WebSocket message received $raw_message")
  try
    client.handler(raw_message)
  catch e
    error(string(e))
  end
end

function send(client::WebSocketClient, data::AbstractString)
  client.debug && info("Sending WebSocket message $data")
  client.socket[:send]( data )
end

function WebSocketClient(host, port, handler::Function; debug = false)
  url = "ws://$host:$port/"
  info("Connecting to $url")
  client = WebSocketClient(nothing, handler, debug)
  client.socket = websocket.WebSocketApp(url, on_message = (ws, ms)->on_ws_message(client, ms, ),
										 on_error   = on_ws_error)
  return client
end

function run(host::AbstractString, port::Integer, handler::Function, dbg = false)
  ws = WebSocketClient(host, port, handler, debug = dbg)
  @spawn ws.socket[:run_forever]()
  return ws
end

end