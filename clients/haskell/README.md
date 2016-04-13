# Space Tyckiting Haskell client

![logo](logo.png)

*Author: Oleg Grenrus*

## Synopsis

Using [stack](http://haskellstack.org/)

```sh
stack setup
stack build
stack exec -- tyckiting-client
```

If you want to use GHC 7.8.4 instead of 7.10.3, `export STACK_YAML=stack-lts-2.yaml`.

## Prerequisites

[Download and install stack](http://docs.haskellstack.org/en/stable/README/#how-to-install).

## How-to start with new AI?

There are few AIs pre-made in `src/Tyckiting/AI/` folder. The pattern is copy-paste:
- copy `Dummy.hs` to `YourBot.hs`
- rename module name and exported ai name
- add your AI to the list in `Main.hs` (first one is the default).
- Run your custom AI with `cabal run -- --ai yourai`


## Testing

The tests are in `test/Tests.hs`, follow the example and add your owns tests.
`Tests.hs` is an ordinary Haskell program, so you can divide it into modules if it grows too big.


## Editors

You should **really** consider tuning your Vim, Emacs or SublimeText.

Then you can skip `cabal install` lines in following guides:

- For Vim follow: http://www.stephendiehl.com/posts/vim_2016.html
- For Emacs follow: https://github.com/serras/emacs-haskell-tutorial/blob/master/tutorial.md
- For Sublime install: https://github.com/SublimeHaskell/SublimeHaskell

## Learning materials

- [LYAH - Learn You a Haskell for Great Good!](http://learnyouahaskell.com/)
- [RWH - Real World Haskell](http://book.realworldhaskell.org/) (2006 &ndash; things are changed)
- [beginners@haskell.org](https://mail.haskell.org/mailman/listinfo/beginners)
- [Learning Haskell - Haskell Wiki](https://wiki.haskell.org/Learning_Haskell)

### optparse-applicative

- [Getting started](https://github.com/pcapriotti/optparse-applicative#getting-started)
- [24 Days of Hackage: optparse-applicative](https://ocharles.org.uk/blog/posts/2012-12-17-24-days-of-hackage-optparse-applicative.html)
- [Thoughbot: Applicative Options Parsing in Haskell](https://robots.thoughtbot.com/applicative-options-parsing-in-haskell)
