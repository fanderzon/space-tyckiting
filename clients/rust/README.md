# Space Tyckiting Rust client

![Rust logo](rust.png)

*Author: Daniel Landau*

## Synopsis
```
cargo build
cargo run -- # [options]
```
## Prerequisites

[Rust](http://www.rust-lang.org/)

If you don't have qualms with running scripts from the Internet without checking, you can.

```
curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --date=2015-04-24
```

## Setup with Docker

```
sudo docker build -t rust-tyckiting .
sudo docker run -ti -v $(pwd)/src:/source/src rust-tyckiting bash
```

And then follow from the top.

### Prerequisites with Docker

- [Docker](https://www.docker.com/)
- If you don't have a suitable kernel running (e.g. you are on OS X or Windows) you might want to check out [Docker machine](https://docs.docker.com/machine/).

## How to start with a new AI

Take a look at `src/ai/mod.rs`. There is a `RandomAI` that does random stuff which you can take as an example to copy the structure. You have to add your AI also to the function `from_name` in the same file so it becomes user selectable.

## Testing

You can add your tests as functions with #[test] before them and run all of them with

```
cargo test
```

There are some examples of tests in the code base.

## Editors

At least Vim and Emacs have packages in the usual places. Sublime Text seems to have one too. Online search engine of your choice is your friend.

## Learning materials

- [Rust by example](http://rustbyexample.com/)
- [Official documentation](http://doc.rust-lang.org/)
- [The official Rust book](http://doc.rust-lang.org/book/)
- [API reference](http://doc.rust-lang.org/std/)
- [Are we web yet?](http://arewewebyet.com/)
- [Fearless Concurrency with Rust](http://blog.rust-lang.org/2015/04/10/Fearless-Concurrency.html)
