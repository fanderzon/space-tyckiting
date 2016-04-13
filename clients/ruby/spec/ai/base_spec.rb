require File.expand_path('../../../ai/base', __FILE__)
Dir[File.expand_path('../../lib/*', __FILE__)].each {|file| require file }

RSpec.describe BaseAI do
  describe "#in_field?" do
    before(:each) do
      config = GameConfig.new(fieldRadius: 2)
      @base = BaseAI.new
      @base.join(2, config)
    end

    it "returns true if the position is in the field" do
      position = Pos.new x: 1, y: -1
      expect(@base.in_field?(position)).to be true
    end

    it "returns false if the position is not in the field" do
      position = Pos.new x: 3, y: 2
      expect(@base.in_field?(position)).to be false
    end
  end
end
