# Space Tyckiting Julia Client

![logo](logo.png)

*Author: ngc92*

## Synopsis

```sh
# install julia packages
julia ./setup.jl

# Run
julia ./cli.jl
```

## Prerequisites

Julia 4.x (http://julialang.org/). 
Uses PyCall for accessing the python websocket package (websocket-client, same as for the python client), 
so the python distribution julia is using needs to have this package installed.
```

## How-to start with new AI?

There is a dummy AIs available in `tyckiting_client/ai/` folder that can be used as a template.
 1. Copy `dummy.jl` to `your_ai.jl`
 2. Go through the comments in that file to find out about which methods to implement.
 3. Run your custom AI with `julia ./cli.jl --ai your_ai`

For testing the AI, it might be helpful to start cli.jl in a julia interpreter and then just call 
the start_client() function to avoid the startup time of the julia interpreter.
```julia
# load the client once, Currently, this runs dummy.jl
include("cli.jl")
# set the ai argument to your bot
args["ai"] = "YOUR_BOT_NAME"
# and call this as often as you like to test your client.
start_client()
```

## Testing

Currently there are no tests defined.
