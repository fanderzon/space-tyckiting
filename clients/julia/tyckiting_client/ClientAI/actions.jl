abstract AbstractAction
botid(a::AbstractAction) = a.bot_id
name(a::AbstractAction) = a.name
position(a::AbstractAction) = a.pos

#########################################################
#  actions that are send to the server
#########################################################

const VALID_ACTIONS = ["move", "radar", "cannon"]

immutable PosAction <: AbstractAction
	bot_id::Int
	pos::Position
	name::ASCIIString
	function PosAction(id, pos, name)
		@assert name ∈ VALID_ACTIONS
		new(id, pos, name)
	end
end

function to_dict(a::PosAction)
	return Dict("botId" => botid(a), "type" => name(a), "pos" => position(a))
end

function make_action(bot_id::Int, x::Int, y::Int, name)
	return PosAction(bot_id, Position(x, y), name)
end

function make_action(bot_id::Int, p::Position, name)
	return PosAction(bot_id, p, name)
end

function MoveAction(args...)
	return make_action(args..., "move")
end

function RadarAction(args...)
	return make_action(args..., "radar")
end

function CannonAction(args...)
	return make_action(args..., "cannon")
end

#########################################################
#  action type for planning that has a associated 
#  rating field for its rating
#########################################################

immutable ActionPlan <: AbstractAction
  name::ASCIIString
  pos::Position
  rating::Float64 # rating of this action
  function ActionPlan(name, pos, rating)
    @assert name ∈ VALID_ACTIONS
    new(name, pos, rating)
  end
end
botid(a::ActionPlan) = error("Action plans do not yet have an associated bot!")

# convert ActionPlan to an actual action
make_action(a::ActionPlan, bot::Int) = make_action(bot, position(a), name(a))
make_action(a::ActionPlan, bot::AbstractBot) = make_action(a, botid(bot))

# generate a bunch of actions plans, all actions of type name, at positions pos with ratings rating
plan_actions(name::ASCIIString, pos::Vector{Position}, rating::Vector{Float64}) = [ActionPlan(name, p, w) for (p,w) in zip(pos, rating)]

# returns a randomized copy of the actions plans, were ratings are shifted by [0, rmax].
randomize(old::Vector{ActionPlan}, rmax::Real) =  map(t->ActionPlan(t.name, t.pos, t.rating + rand() * rmax), old)

# new action plans based on old ones, just shifting weights
plan_actions(old::Vector{ActionPlan}, shift::Real) = map(t->ActionPlan(t.name, t.pos, t.rating + shift), old)
function plan_actions(old::Vector{ActionPlan}, shift::Vector{Float64})
  result =  map(x->ActionPlan(x[1].name, x[1].pos, x[1].rating + x[2]), zip(old, shift))
  return convert(Vector{ActionPlan}, result)
end

# gets the best action 
best_action(actions::Vector{ActionPlan}, N::Integer = 1) = sort(actions, by = a->a.rating, rev=true)[1]