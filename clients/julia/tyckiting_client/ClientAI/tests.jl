include("ClientAI.jl")
using ClientAI
using Base.Test

# Test basic coordinate type stuff
a = HexGridCoordinate(1.5, 2.5)
@test a.x == 1.5
@test a.y == 2.5

b = Position(1, 2)
d = Dict("x"=>1, "y"=>2)
@test b.x == 1 && b.y == 2
@test b == HexGridCoordinate(1, 2)
@test b == Position(d)
@test to_dict(b) == d

@test zero(Position) == Position(0,0)

# cube coordinates
@test HexGridCoordinate(CubeGridCoordinate(b)) == b

# test distance function
@test distance(Position(0,0), Position(1,0)) == 1
@test distance(Position(0,0), Position(0,1)) == 1
@test distance(Position(0,0), Position(-1,1)) == 1

# grid iteration
area = GeneralCircularArea(Position(1,1), 0)
@test collect(area) == [Position(1,1)]
area = GeneralCircularArea(Position(0,0), 1)
@test collect(area) == [Position(-1,0), Position(-1,1), Position(0,-1), Position(0,0), Position(0,1), Position(1,-1), Position(1,0)]
area = GeneralCircularArea(Position(0,0), 3)
@test length(area) == 37
for p in area
  @test distance(p, Position(0,0)) <= radius(area)
end

i = intersect(GeneralCircularArea(Position(1,0), 1), OriginCircularArea(1))
@test collect(i) == [Position(0,0), Position(0,1), Position(1, -1), Position(1, 0)]

@test length(collect(intersect(circle(Position(0,0), 2), circle(10)))) == length(collect(circle(Position(0,0), 2)))

