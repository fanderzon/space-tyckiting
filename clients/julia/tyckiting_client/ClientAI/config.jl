immutable Config
  bots::Int
  field_radius::Int
  move::Int
  start_hp::Int
  cannon::Int
  radar::Int
  see::Int
  max_count::Int
  loop_time::Int
  function Config(bots::Integer=3, fieldRadius::Integer=14, move=2, startHp=10, cannon=1,
                 radar=3, see=2, maxCount=200, loopTime=300)
    new(bots, fieldRadius, move, startHp, cannon, radar, see, maxCount, loopTime)
  end
end

function Config(dict::Dict)
  params = [dict["bots"], dict["fieldRadius"], dict["move"], dict["startHp"], dict["cannon"], dict["radar"],
                dict["see"], dict["maxCount"], dict["loopTime"]]
  params = map(Int, params)
  return Config(params...)
end

