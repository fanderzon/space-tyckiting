class Pos
  attr_accessor :x, :y

  def initialize(hash)
    self.x = hash[:x]
    self.y = hash[:y]
  end

  def to_s
    "(#{x}, #{y})"
  end

  def as_json
    {x: x, y: y}
  end

  def distance_from(position)
    [
      (@x - position.x).abs,
      (@y - position.y).abs,
      (@x + @y - position.x - position.y).abs
    ].max
  end

  def neighbours(distance)
    (-distance..distance).inject([]) do |fields, dx|
      ([-distance, -dx-distance].max..[distance, -dx+distance].min).inject(fields) do |_fields, dy|
        _fields << Pos.new(x: @x + dx, y: @y + dy)
      end
    end
  end

  def ==(position)
    @x == position.x && @y == position.y
  end
end
