using ArgParse

include("tyckiting_client/client.jl")

global args

function main()
  # Main method for running the client. Reads configuration from command line,
  # and starts the client.
  s = ArgParseSettings(description="Destroy 'em all")
  @add_arg_table s begin
    "--host", "-H"
      default="localhost"
      help="Host to connect to"
    "--port", "-P"
      default=3000
      arg_type=Int
      help="Port to connect to"
    "--verbose", "-v"
      action=:store_true
      help="Verbose output"
    "--name", "-n"
      default="bot"
      help="Bot's name"
    "--ai", "-a"
      default="dummy"
      help="Ai package"
  end

  global args = parse_args(s)
  start_client()
end

function start_client()
  global args
  client = TykitingClient(args["host"], args["port"], args["name"], args["ai"])
  @sync run(client, args["verbose"])
end

main()
