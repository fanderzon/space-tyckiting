require File.expand_path('../../ai/base',  __FILE__)

class AI < BaseAI
  def move(bots_data, events)
    update_bots bots_data
    print_bots

    actions = []
    actions = handle_events(actions, events)
    actions = move_bots(actions)

    save_to_history actions

    actions.map(&:as_json)
  end

  def handle_events(actions, events)
    events.inject([]) do |actions, event|
      if event.event == "see"
        if event.pos.distance_from(find_bot_by_id(event.source).pos) > 1
          actions << Action.new(botId: event.source, type: "cannon", pos: event.pos)
        end
      end

      actions
    end
  end

  def move_bots(actions)
    bot_ids = actions.map(&:botId)

    @bots.each do |botId, bot|
      if bot.alive && !bot_ids.include?(botId)
        actions = move_bot actions, bot
      end
    end

    actions
  end

  def move_bot(actions, bot)
    new_position = bot.pos.neighbours(@config.move).sample

    if in_field?(new_position)
      actions << Action.new(botId: bot.botId, type: "move", pos: new_position)
    else
      move_bot(actions, bot)
    end
  end
end
