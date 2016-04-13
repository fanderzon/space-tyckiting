module Dummy_AI

using ClientAI

# the DummyAI class
type DummyAI <: AbstractAI
  config::Config
  bots::Vector{AbstractBot}
end

# This function is called when the game starts and gets information about own and enemy bots.
function on_start(ai::DummyAI, own_bots::Vector, enemy_bots::Vector)
end

# This function is called at the beginning of every round, and gets information about own bots and
# the events that happend in the last round. It is called before events are processed by the 
# on_event handlers.
function init_round(ai::DummyAI, bots::Vector{AbstractBot}, events::Vector{AbstractEvent}, round_id::Integer)
  ai.bots = bots
end

# this function is called after event handling and has to return an array of actions that the bot wants to
# perform.
function decide(ai::DummyAI)
  bots = filter_valid(ai.bots)
  actions = AbstractAction[]
  for b in bots
    push!(actions, MoveAction(botid(b), rand(collect(move_area(b, ai.config)))))
  end
  return actions
end

# ignore all events
ClientAI.on_event(ai::DummyAI, event::AbstractEvent) = false

# this function is called from main to create the AI
function create(team_id, config::Config)
	info("create ai $team_id")
	return DummyAI(config, AbstractBot[])
end

end
