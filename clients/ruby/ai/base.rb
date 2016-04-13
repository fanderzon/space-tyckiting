require File.expand_path('../../lib/message',  __FILE__)
require File.expand_path('../../lib/action',  __FILE__)
require File.expand_path('../../lib/bot',  __FILE__)

class BaseAI
  attr_accessor :teamId, :config, :bots

  def join(teamId, config)
    self.teamId = teamId
    self.config = config
  end

  def start(bots_data)
    self.bots = bots_data.inject({}) do |bots, bot_data|
      bots[bot_data.botId] = Bot.new(bot_data)
      bots
    end
  end

  def save_to_history(actions)
    actions.each do |action|
      @bots[action.botId].update_history(action)
    end
  end

  def update_bots(bots_data)
    bots_data.each do |bot_data|
      @bots[bot_data.botId].update(bot_data)
    end
  end

  def print_bots
    @bots.each do |id, bot|
      puts bot.to_s
    end
  end

  def find_bot_by_id(id)
    @bots[id]
  end

  def in_field?(position)
    Pos.new(x: 0, y: 0).distance_from(position) <= @config.fieldRadius
  end
end
