require File.expand_path('../pos',  __FILE__)

class BotData
  attr_accessor :botId, :name, :teamId, :alive, :pos, :hp

  def initialize(hash)
    self.botId = hash[:botId]
    self.name = hash[:name]
    self.teamId = hash[:teamId]
    self.alive = hash[:alive]
    self.pos = Pos.new hash[:pos]
    self.hp = hash[:hp]
  end
end
