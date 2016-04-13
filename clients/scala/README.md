# Space Tyckiting Scala client

Space Tyckiting client for the [Scala language](http://www.scala-lang.org/).

![logo](logo.png)

*Author: Jukka Viinamäki*

## Getting started

**Prerequisites**

You'll need Java and [SBT](http://www.scala-sbt.org/) 0.13.x pre-installed.

**Developing your AI**

The Scala client is divided into two top level packages:

* `ai`
* `tyckiting`

The `tyckiting` package contains the client framework and command line tools. The `ai` package is where you'll develop your very own AI.

Simply use the existing `MyScalaAi` class as a template or create your own `object` which extends the `Tyckiting` trait. Implement the `makeDecisions` method with a clever logic for your AI!

While developing your bot, you can run it from the SBT console simply with `run`. It will connect by default to localhost:3000, expecting to find the Space Tyckiting server there.

## Building for distribution

Once your AI is ready for distribution, you can build a standalone JAR file from it.

Run `sbt assembly` (or just `assembly` from the SBT console). A stand-alone distributable JAR package, tyckiting.jar, will be generated to target/scala-2.11.

This can be run in the usual way, `java -jar target/scala-2.11/tyckiting.jar`, and contains its own command line parser.
