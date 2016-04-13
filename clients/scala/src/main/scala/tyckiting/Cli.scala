package tyckiting

import scopt._

trait Cli { self: Ai =>
    
  val parser = new OptionParser[TyckitingConfig]("tyckiting-scala") {
    head("tyckiting-scala", "1.0")
    
    opt[String]('h', "host") valueName("<host>") action { (x, c) =>
      c.copy(host = x) 
    } text("Hostname or IP address of the server")
    
    opt[Int]('p', "port") valueName("<port>") action { (x, c) =>
      c.copy(port = x)
    } text("Port on the server")
  }
    
  def main(args: Array[String]) {
    parser.parse(args, TyckitingConfig()) match {
      case Some(config) => runClient(config)
      case None =>
    }
  }
  
  def runClient(config: TyckitingConfig) {
    new WsClient(config, self).startClient
  }
}

case class TyckitingConfig(
  host: String = "localhost",
  port: Int = 3000)
