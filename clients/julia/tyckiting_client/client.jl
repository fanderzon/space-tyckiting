import JSON

include("ClientAI/ClientAI.jl")
using ClientAI

include("messages.jl")
include("websocket.jl")

type TykitingClient
  host::ASCIIString
  port::Int
  name::ASCIIString
  ai_source::AbstractString
  ai::Nullable{AbstractAI}
  ai_module::Module
  team_id::Int
  socket
  function TykitingClient(host="localhost", port=3000, name="bot", ai="ngcbot")
    # load the ai script
    ai_source = "tyckiting_client/ai/"*ai*".jl"
    # check already here, that the file exists
    @assert isfile(ai_source)
    ai_module = nothing
    @checked ai_module = include(ai_source)

    # ensure compilation of the bot functions
    dummy = ai_module.create(0, Config())
    info("running bot in dummy computation to compile")
    @time begin
      # TODO make events believable! This sight event is probably not possible, so it might cause errors for bots
      # that rely on that.
      events = AbstractEvent[SightEvent(1, 0, Position(0,0)), NoActionEvent(0)]
      ai_module.init_round(dummy, AbstractBot[ClientAI.WebSockOwnBot(0, "dummy", 0, true, Position(0,0), 10)], events, 0)
      event_dispatch(dummy, events)
      ai_module.decide(dummy)
    end

    new(host, port, name, ai_source, nothing, ai_module, 0, nothing)
  end
end

function on_connected(client::TykitingClient, message::ConnectedMsg)
  info("Connected to server with id $(message.team_id)")
  client.team_id = message.team_id

  @checked client.ai = client.ai_module.create(message.team_id, message.config)
  send(client, Dict("type" => "join", "teamName" => client.name))
end

function on_start(client::TykitingClient, message::StartMsg)
#   Handles game start event from server.
#    Args:  message: Message from server containing team compositions
  info("Game started")
  if isdefined(client.ai_module, :on_start)
    enemies = vcat([t.bots for t in message.other_teams]...)
    client.ai_module.on_start(get(client.ai), message.you.bots, enemies)
  end
end

function on_events(client::TykitingClient, message::EventsMsg)
  info("Round $(message.round_id)")
  responses = Any[]
  timing = @elapsed try
    # event dispatcher loop
    client.ai_module.init_round(get(client.ai), message.you.bots, message.events, message.round_id)
    event_dispatch(get(client.ai), message.events)
    responses = client.ai_module.decide(get(client.ai))
  catch e
    println(e)
    Base.show_backtrace(STDOUT, catch_backtrace())
  end
  if timing > 0.25
    warn("Computation took longer than 250ms ($(round(Int, 1000*timing))). Limit: 300ms!")
  end

  actions = Dict("type" => "actions", "roundId" => message.round_id, "actions" => map(to_dict, responses))
  send(client, actions)
end

function on_end(client::TykitingClient, message::EndMsg)
  # Handles game end event from server
  #    Args:  message: Message from server containing the winning team id
  if message.winner_team_id == -1
      result = "It's a draw."
  elseif message.winner_team_id == client.team_id
      result = "You win!"
  else
    result = "You loose."
  end

  info("Game ended. $result")
  close(client)
end

function on_message(client::TykitingClient, message::AbstractString)
  json = JSON.parse(message)
  msgtype = json["type"]
  if msgtype == "connected"
    on_connected(client, ConnectedMsg(json))
  elseif msgtype == "start"
    on_start(client, StartMsg(json))
  elseif msgtype == "events"
    on_events(client, EventsMsg(json))
  elseif msgtype == "end"
    on_end(client, EndMsg(json))
  elseif msgtype == "error"
    error(json)
  else
    warn("Unkown message type $msgtype")
  end
end

function run(client::TykitingClient, debug)
  ws = WebSock.run(client.host, client.port, x->on_message(client, x), debug)
  client.socket = ws
  return
end

function send(client::TykitingClient, data::Dict)
  js = JSON.json(data)
  WebSock.send(client.socket, js)
end
close(client::TykitingClient) = WebSock.close(client.socket)
