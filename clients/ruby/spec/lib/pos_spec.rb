require File.expand_path('../../../lib/pos', __FILE__)

RSpec.describe Pos do
  describe "#distance_from" do
    it "calculates the distance between the current and the given position" do
      position_a = Pos.new(x: 1, y: -1)
      position_b = Pos.new(x: 3, y: 4)
      expect(position_a.distance_from(position_b)).to eql(7)
    end
  end

  describe "#neighbours" do
    it "returns all available pos with a given max distance" do
      position = Pos.new(x: 1, y: -1)
      expect(position.neighbours(2)).to eq([
        Pos.new(x: -1, y: -1), Pos.new(x: -1, y: 0), Pos.new(x: -1, y: 1),
        Pos.new(x: 0, y: -2), Pos.new(x: 0, y: -1), Pos.new(x: 0, y: 0), Pos.new(x: 0, y: 1),
        Pos.new(x: 1, y: -3), Pos.new(x: 1, y: -2), Pos.new(x: 1, y: -1), Pos.new(x: 1, y: 0), Pos.new(x: 1, y: 1),
        Pos.new(x: 2, y: -3), Pos.new(x: 2, y: -2), Pos.new(x: 2, y: -1), Pos.new(x: 2, y: 0),
        Pos.new(x: 3, y: -3), Pos.new(x: 3, y: -2), Pos.new(x: 3, y: -1)
      ])
    end
  end
end
