module ClientAI
  import Base: position

  include("config.jl")
  include("hexgrid.jl")
  include("bot.jl")
  include("actions.jl")
  include("events.jl")
  include("ai.jl")

  macro checked(ex)
    return quote
      try
        $(esc(ex))
      catch e
        showerror(STDERR, e)
        Base.show_backtrace(STDOUT, catch_backtrace())
      end
    end
  end
  export @checked

	export AbstractAI

  # general functions and types
  export AbstractAI, Position, Config
  export botid, position, to_dict

  # events
  export AbstractEvent, HitEvent, DeathEvent, SightEvent, RadarEvent, DetectionEvent, DamageEvent, MoveEvent, NoActionEvent
  export on_event, event_dispatch

  # actions
  export AbstractAction, MoveAction, RadarAction, CannonAction
  export make_action, plan_actions, best_action, randomize

  # bot functions
  export AbstractBot
  export is_alive, team, name, hitpoints, filter_valid

  # grid functions
  export distance, radius, center, circle
  export view_area, radar_area, move_area, damage_area, map_area
end
