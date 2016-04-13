name := "tyckiting-scala-client"

scalaVersion := "2.11.6"

assemblyJarName in assembly := "tyckiting.jar"

resolvers += "Typesafe releases" at "http://repo.typesafe.com/typesafe/releases/"

fork in run := true

libraryDependencies ++= Seq(
  "com.github.scopt" %% "scopt" % "3.3.0",
  "io.backchat.hookup" %% "hookup" % "0.3.0",
  "com.typesafe.play" %% "play-json" % "2.3.8")
