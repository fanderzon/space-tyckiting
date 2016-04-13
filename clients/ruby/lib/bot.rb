require 'colorize'

class Bot
  attr_accessor :history, :pos, :hp, :alive, :botId, :name

  def initialize(bot_data)
    self.history = []
    self.pos = bot_data.pos
    self.hp = bot_data.hp
    self.alive = bot_data.alive
    self.botId = bot_data.botId
    self.name = bot_data.name
  end

  def update(bot_data)
    self.pos = bot_data.pos
    self.hp = bot_data.hp
    self.alive = bot_data.alive
  end

  def update_history(action)
    self.history << action
  end

  def to_s
    "Bot: #{botId.to_s.colorize(:light_blue)}, position: #{pos.to_s.colorize(:light_blue)}, " +
      "hp: #{hp.to_s.colorize(:light_blue)}, alive: #{alive.to_s.colorize(:light_blue)}."
  end
end
