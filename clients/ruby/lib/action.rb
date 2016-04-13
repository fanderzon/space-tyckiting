require File.expand_path('../pos',  __FILE__)

class Action
  attr_accessor :botId, :type, :pos

  def initialize(hash)
    self.botId = hash[:botId]
    self.type = hash[:type]
    self.pos = if hash[:pos].respond_to?(:x)
      hash[:pos]
    else
      Pos.new(hash[:pos])
    end
  end

  def as_json
    {botId: botId, type: type, pos: pos.as_json}
  end
end
