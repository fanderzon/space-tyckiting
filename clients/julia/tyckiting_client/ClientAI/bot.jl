abstract AbstractBot

# generic bot functions
is_alive(b::AbstractBot) = b.alive
team(b::AbstractBot) = b.team_id
botid(b::AbstractBot) = b.bot_id
name(b::AbstractBot) = b.name
# these might fail if they are not known
position(b::AbstractBot) = b.pos
hitpoints(b::AbstractBot) = b.HP

################################################
# bot types:
# own bot: we know everything about our own bots
type WebSockOwnBot <: AbstractBot
  bot_id::Int
  name::ASCIIString
  team_id::Int
  alive::Bool
  pos::Position
  HP::Int
end

# enemy bots: position and hp are unknown
type WebSockEnemyBot <: AbstractBot
  bot_id::Int
  name::ASCIIString
  team_id::Int
  alive::Bool
end

position(b::WebSockEnemyBot) = error("position of enemy bots are not known!")
hitpoints(b::WebSockEnemyBot) = error("hitpoints of enemy bots are not known!")

###################################################
# make a bot from the dict of the websocket message
function make_bot(d::Dict)
  # common data
  id = Int(d["botId"])
  name = d["name"]
  team = Int(d["teamId"])
  alive = Bool(d["alive"])

  if haskey(d, "pos")
    return WebSockOwnBot(id, name, team, alive, Position(d["pos"]), d["hp"])
  else
    return WebSockEnemyBot(id, name, team, alive)
  end
end

