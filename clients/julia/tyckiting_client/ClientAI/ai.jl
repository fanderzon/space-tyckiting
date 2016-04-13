abstract AbstractAI

# event dispatcher loop: calls on event for all events
# in an event vector
function event_dispatch(ai, events::Vector{AbstractEvent})
  for event in events
    on_event(ai, event)
  end
end


# default event handlers: warn for important events that they are missed.
function on_event(ai::AbstractAI, event::AbstractEvent)
  if typeof(event) == NoActionEvent
     warn("Bot $(botid(event)) did not act last round!")
  elseif typeof(event) == MoveEvent
    # nothing happens
  else
    warn("Event of type $(typeof(event)) not processed by $(typeof(ai))! Implement ClientAI.on_event")
  end
end

# filter out all dead bots
filter_valid{T <: AbstractBot}(bots::Vector{T}) = filter(is_alive, bots)

