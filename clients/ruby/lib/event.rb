require File.expand_path('../pos',  __FILE__)

class Event
  attr_accessor :event, :botId, :pos, :source, :damage

  def initialize(hash)
    self.event = hash[:event]
    self.botId = hash[:botId]
    self.source = hash[:source]
    self.damage = hash[:damage]
    self.pos = Pos.new hash[:pos] if hash[:pos]
  end
end
