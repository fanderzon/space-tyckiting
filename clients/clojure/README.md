# Space Tyckiting Clojure client

A Clojure client skeleton for the Futurice CodeCamp 2015.

![logo](logo.png)

*Author: Nils Blom-Oeste*

## Synposis

```sh
# Join game
lein run -H <host> -P <port> -a <ai-name>

# Get CLI help
lein run`
```

You need to provide all three parameters to start the client! To get started,
you can use the example AI called "random" by using `-a random`.

### Prerequisities

- [Java JDK](http://www.oracle.com/technetwork/java/javase/downloads/index.html) version 6 or later
- [Leiningen](https://github.com/technomancy/leiningen)

Clojure and its dependencies will be installed when you run a lein task automatically.
`lein deps` would install them explicitly.

Use `lein -h` to get help for Leiningen.

## How-to start with new AI

An example AI can be found in `src/tyckiting/ai/random.clj`.
To start a new AI, copy the above file to a new name, adjust it and pass it's
name during program start as CLI param.

### AI Requirements

AIs need to be in the namespace `tyckiting.ai.<AI-NAME>` where you can choose
`<AI-NAME>` as you wish, e.g. `tyckiting.ai.foobar`. To use this AI pass the
name to the `-a` param on start, e.g. `lein run -H tyckiting.futurice.com -P 4028 -a foobar`.

Evert AI needs to have a function called `create-actions`. This function gets
called every turn with the current game-state as a map. It needs to return the
actions for the next round. You can make use of `tyckiting.actions` to create
valid actions. Once again: Check out the example `tyckiting.ai.random`.

## Testing

An example test can be found in `test/tyckiting/ai_test.clj`. More tests can be
added in that directory. Use the existing test as boilerplate if you wish.

- `lein test` runs all tests

## Editors

Of course you can write Clojure with any editor. However, Clojure development
usually involves usage of the Clojure REPL and if you want that too, you should
look for an editor with proper support for the REPL workflow (it's not a hard
requirement though).

All of the following have good REPL support.

### Popular editors
- Emacs with [Cider](https://github.com/tpope/vim-fireplace)
- Vim with [vim-fireplace](https://github.com/tpope/vim-fireplace)

### Popular IDEs
- [LightTable](http://lighttable.com/)
- IntelliJ [Cursive](https://cursiveclojure.com/)
- [Ecplise]() with [Counterclockwise](http://doc.ccw-ide.org/)

## Learning materials

- [Why Clojure?](http://clojure.org/rationale)
- [Official Getting Started Guide](http://clojure.org/getting_started) with
  Links to Tools and Editors, Books and various helpful Resources
- [API Documentation](http://clojuredocs.org/quickref) and
  [Cheatsheet](http://clojure.org/cheatsheet)
- [tryclj](http://www.tryclj.com/) - Try Clojure online without installing
  anything
- [4 Clojure](https://www.4clojure.com/) - Learn Clojure by solving small coding
  tasks
- [Clojure Koans](http://clojurekoans.com/) - become a Clojure and Zen master!

## Beyond the JVM
- Clojure compiles to Javascript as
  [ClojureScript](http://clojure.org/clojurescript). One awesome way to share code
  between backend and frontend is [cljs](https://github.com/lynaghk/cljx).
- Clojure can also be used on the [.NET CLR](http://clojure.org/clojureclr).
  Why not use it for [Unity 3D game development](https://www.youtube.com/watch?v=tJr_TD1BtF0)?

## License

Copyright © 2015 Futurice

Distributed under the Eclipse Public License either version 1.0 or (at
your option) any later version.
