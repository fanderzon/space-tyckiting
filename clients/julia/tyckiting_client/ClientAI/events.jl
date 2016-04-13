abstract AbstractEvent
abstract PositionalEvent <: AbstractEvent

immutable HitEvent <: AbstractEvent
  bot_id::Int
  source::Int
end

immutable DeathEvent <: AbstractEvent
  bot_id::Int
end

immutable SightEvent <: PositionalEvent
  bot_id::Int
  source::Int
  pos::Position
end

immutable RadarEvent <: PositionalEvent
  pos::Position
end
botid(event::RadarEvent) = error("Radar event is not bound to specific bot.")

immutable DetectionEvent <: AbstractEvent
  bot_id::Int
end

immutable DamageEvent <: AbstractEvent
  bot_id::Int
  damage::Int
end

immutable MoveEvent <: PositionalEvent
  bot_id::Int
  pos::Position
end

immutable NoActionEvent <: AbstractEvent
  bot_id::Int
end

const EventTypeMap = Dict("hit" => HitEvent,
                      "die" => DeathEvent,
                      "see" => SightEvent,
                      "radarEcho" => RadarEvent,
                      "detected" => DetectionEvent,
                      "damaged" => DamageEvent,
                      "move" => MoveEvent,
                      "noaction" => NoActionEvent)

const EventNameMap = Dict(zip(values(EventTypeMap), keys(EventTypeMap)))

const EventMemberToID  = Dict(:bot_id => "botId", :source => "source", :damage => "damage", :pos => "pos")
const EventMemberTypes = Dict(:bot_id => Int, :source => Int, :damage => Int, :pos => Position)

# event functions
name(event::AbstractEvent) = EventNameMap[typeof(event)]
position(event::PositionalEvent) = event.pos
botid(event::AbstractEvent) = event.bot_id

function make_event(d::Dict)
  # get the event type
  etype = EventTypeMap[d["event"]]

  # find members and member types of the corresponding type
  member_symbols = fieldnames(etype)
  keys = [EventMemberToID[f] for f in member_symbols]
  types = [EventMemberTypes[f] for f in member_symbols]

  # and call the constructor with these dynamical arguments
  return etype( [t(d[k]) for (k, t) in zip(keys, types)]... )
end
