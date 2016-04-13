################################################################
# collect the data for a team
################################################################
immutable Team
  name::ASCIIString
  team_id::Int
  bots::Vector{AbstractBot}
end

function Team(d::Dict)
  return Team(d["name"], Int(d["teamId"]), map(ClientAI.make_bot, d["bots"]))
end

###############################################################
#   data types that encapsulate network messages
###############################################################
abstract AbstractMessage

immutable ConnectedMsg <: AbstractMessage
  team_id::Int
  config::Config
end

function ConnectedMsg(d::Dict)
  return ConnectedMsg(Int(d["teamId"]),  Config(d["config"]))
end

immutable StartMsg <: AbstractMessage
  you::Team
  other_teams::Vector{Team}
end

function StartMsg(d::Dict)
  return StartMsg(Team(d["you"]),  map(Team, d["otherTeams"]))
end

immutable EndMsg <: AbstractMessage
  winner_team_id::Int
end

function EndMsg(d::Dict)
  winner = d["winnerTeamId"]
  if winner == nothing
    winner = -1
  else
    winner = Int(winner)
  end
  return EndMsg(winner)
end

type EventsMsg <: AbstractMessage
  round_id::Int
  you::Team
  other_teams::Vector{Team}
  events::Vector{AbstractEvent}
  config::Config
end

function EventsMsg(d::Dict)
  rid = Int(d["roundId"])
  you = Team(d["you"])
  config = Config(d["config"])
  others = map(Team, d["otherTeams"])
  events = convert(Vector{AbstractEvent}, map(ClientAI.make_event, d["events"]))
  return EventsMsg(rid, you, others, events, config)
end
