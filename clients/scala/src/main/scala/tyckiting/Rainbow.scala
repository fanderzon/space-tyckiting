package tyckiting

import scala.Console._

trait Rainbow {
  
  lazy val isANSISupported = {
    Option(System.getProperty("sbt.log.noformat")).map(_ != "true").orElse {
      Option(System.getProperty("os.name"))
        .map(_.toLowerCase)
        .filter(_.contains("windows"))
        .map(_ => false)
    }.getOrElse(true)
  }

  implicit class Colors(sc: StringContext) {
    def red(args: Any*): String = if (isANSISupported) (RED + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def blue(args: Any*): String = if (isANSISupported) (BLUE + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def cyan(args: Any*): String = if (isANSISupported) (CYAN + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def green(args: Any*): String = if (isANSISupported) (GREEN + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def magenta(args: Any*): String = if (isANSISupported) (MAGENTA + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def white(args: Any*): String = if (isANSISupported) (WHITE + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def black(args: Any*): String = if (isANSISupported) (BLACK + sc.s(args:_*) + RESET) else sc.s(args:_*)
    def yellow(args: Any*): String = if (isANSISupported) (YELLOW + sc.s(args:_*) + RESET) else sc.s(args:_*)
  }
}
