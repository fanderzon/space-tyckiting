abstract AbstractCoordinate
import Base: +

immutable HexGridCoordinate{T} <: AbstractCoordinate
  x::T
  y::T
end

Base.zero{T}(::Type{HexGridCoordinate{T}}) = HexGridCoordinate(zero(T), zero(T))
+{T}(a::HexGridCoordinate{T}, b::HexGridCoordinate{T}) = HexGridCoordinate(a.x + b.x, a.y + b.y)
Base.print(io::IO, c::HexGridCoordinate) = print(io, c.x, ", ", c.y)

# typealias for the almost exclusively used integer grid positions
typealias Position HexGridCoordinate{Int}

function Position(d::Dict)
  return Position(Int(d["x"]), Int(d["y"]))
end

to_dict(p::HexGridCoordinate) = Dict("x" => p.x, "y" => p.y)
# this seems a little strange, but it means that we can now easily build functions
# that takes positional objects or positions as parameters and just call this function to
# get the actual position
position(p::AbstractCoordinate) = p
distance(a::HexGridCoordinate, b::HexGridCoordinate) = distance(CubeGridCoordinate(a), CubeGridCoordinate(b))

# ######################################################
# cube coordinate position helper type for internal
# computations on the hexagonal grid
########################################################
immutable CubeGridCoordinate{T} <: AbstractCoordinate
  x::T
  y::T
  z::T
end

CubeGridCoordinate(p::HexGridCoordinate) = CubeGridCoordinate(p.x, -p.x - p.y, p.y)
HexGridCoordinate(p::CubeGridCoordinate) = HexGridCoordinate(p.x, p.z)
function distance(a::CubeGridCoordinate, b::CubeGridCoordinate)
  return max(abs(a.x-b.x), abs(a.y-b.y), abs(a.z-b.z))
end

Base.zero{T}(::Type{CubeGridCoordinate{T}}) = HexGridCoordinate(zero(T), zero(T), zero(T))

########################################################
# type for circular areas in hex grid
########################################################

abstract AbstractArea
abstract CircularArea <: AbstractArea

# circular area that is centered around the origin
immutable OriginCircularArea <: CircularArea
  radius::Int
end

radius(a::CircularArea) = a.radius
center(a::OriginCircularArea) = Position(0, 0)
circle(r::Int) = OriginCircularArea(r)

Base.in(p::Position, a::CircularArea) = distance(p, center(a)) <= radius(a)

Base.eltype(::Type{AbstractArea}) = Position
Base.start(a::CircularArea) = Position(-radius(a), 0)
Base.length(iter::CircularArea) = 1 + 6div(radius(iter) * (radius(iter) + 1), 2)

function Base.next(iter::CircularArea, state::Position)
  rad = radius(iter)
  if state.y < min(rad, -state.x+rad)
    nxt = Position(state.x, state.y + 1)
  else
    nxt = Position(state.x + 1, max(-rad, -1-state.x-rad))
  end
  return state + center(iter), nxt
end

function Base.done(iter::CircularArea, state::Position)
  return state.x > radius(iter)
end


# this type defines a circular area in a hexagonal grid.
# it is equipped with an iterator interface
immutable GeneralCircularArea <: CircularArea
  center::Position
  radius::Int
end

radius(a::GeneralCircularArea) = a.radius
center(a::GeneralCircularArea) = a.center
circle(o::Position, r::Int) = GeneralCircularArea(o, r)
circle(origin, radius::Int) = circle(position(origin), radius)

# shift circles
+(a::CircularArea, b::Position) = GeneralCircularArea(center(a)+b, radius(a))

# type for intersection of spherical shape and a complete map
immutable IntersectionArea{T1<:AbstractArea, T2 <: AbstractArea} <: AbstractArea
  A::T1
  B::T2
end

immutable IntIterState
  value::Position
  state::Position
  done::Bool
end

function Base.start(iter::IntersectionArea)
  state = start(iter.A)
  value, state = next(iter.A, state)
  while value ∉ iter.B && !done(iter.A, state)
    value, state = next(iter.A, state)
  end
  return IntIterState(value, state, value ∉ iter.B)
end
# cannot predict length

function Base.next(iter::IntersectionArea, state::IntIterState)
  # return the last value now
  if done(iter.A, state.state)
    return state.value, IntIterState(state.value, state.state, true)
  end

  v, n = next(iter.A, state.state)
  while v ∉ iter.B && !done(iter.A, n)
    v, n = next(iter.A, n)
  end
  return state.value, IntIterState(v, n, v ∉ iter.B)
end

Base.done(iter::IntersectionArea, state::IntIterState) = state.done
Base.intersect(a::AbstractArea, b::AbstractArea) = IntersectionArea(a, b)

###################################################
#        convenience functions for bots
###################################################
# centered areas
view_area(config::Config)    = circle(config.see)
radar_area(config::Config)   = circle(config.radar)
move_area(config::Config)    = circle(config.move)
damage_area(config::Config)  = circle(config.cannon)
map_area(config::Config)     = circle(config.field_radius)

# areas around a specific point
view_area(origin,  config::Config)   = intersect(circle(origin, config.see),    map_area(config))
radar_area(origin, config::Config)   = intersect(circle(origin, config.radar),  map_area(config))
move_area(origin,  config::Config)   = intersect(circle(origin, config.move),   map_area(config))
damage_area(origin,  config::Config) = intersect(circle(origin, config.cannon), map_area(config))


