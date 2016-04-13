# Space Tyckiting Ruby client

*Author: Frank Proessdorf*

## Prerequisites

I strongly recommend using rvm (https://rvm.io/rvm/install) or rbenv (https://github.com/sstephenson/rbenv#installation) to easily manage ruby versions. This README however works with the default ruby setup on OS X to make it easier for newcomers.


## Synopsis

```sh
gem install bundler
bundle install --path vendor/bundle
bundle exec ruby client.rb
```

## How-to start with new AI?

There are few AIs pre-made in `ai/` folder.
- copy `dummy.rb` to `yourbot.rb`
- start `client.rb` with `yourbot` as `--ai` option


## Run the tests

```sh
bundle exec rspec
```


## Editors

Most ruby developers seem to use [TextMate](https://macromates.com/) or [SublimeText](http://www.sublimetext.com/). Ruby support comes out of the box with both, RSpec support can be gained here for [TextMate](https://github.com/rspec/rspec.tmbundle) and here for [SublimeText](https://github.com/SublimeText/RSpec).


## Learning materials

- [Why's poignant guide to ruby](http://mislav.uniqpath.com/poignant-guide/book/chapter-1.html)
- [Ruby lang](https://www.ruby-lang.org/en/)
- [Ruby monk](https://rubymonk.com/learning/books/1-ruby-primer)
