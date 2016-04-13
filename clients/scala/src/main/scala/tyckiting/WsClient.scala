package tyckiting

import reads._
import writes._
import io.backchat.hookup._
import play.api.libs.json._
import org.json4s.jackson.JsonMethods._
import java.net._

class WsClient(config: TyckitingConfig, ai: Ai) extends Rainbow {
  
  lazy val client = {
    val uri = URI.create(s"ws://${config.host}:${config.port}")
    new DefaultHookupClient(HookupClientConfig(uri)) {
      
      def receive = {
        case JsonMessage(jv) =>
          Json.parse(compact(jv)).validate[ServerMessage].fold(err =>
            onUnexpected(pretty(jv), Json.prettyPrint(JsError.toFlatJson(err))),
            onMessage)
            
        case Error(t) =>
          logAndExit("Error", t)
          
        case Disconnected(t) =>
          logAndExit("Disconnected", t)
      }
    }
  }
  
  var gameConfig: GameConfig = _
  
  def startClient() {
    log(s"Starting client for ${ai.teamName}")
    client.connect()
  }
  
  def logAndExit(event: String, t: Option[Throwable]) {
    log(s"$event${t.map(t => ": " + red"${t.getMessage}") getOrElse ""}")
    sys.exit
  }
  
  def onUnexpected(js: String, err: String) {
    log(s"Unexpected message: $js, errors: $err")
  }
  
  def onMessage: ServerMessage => Unit = {
    case m: ConnectedMessage => onConnected(m)
    case m: StartMessage => onStart(m)
    case m: EndMessage => onEnd(m)
    case m: EventsMessage => onEvents(m)
  }
  
  def onConnected(m: ConnectedMessage) {
    log(s"Connected to server, teamId: ${m.teamId}")
    gameConfig = m.config
    send(JoinMessage(ai.teamName))
  }
  
  def onStart(m: StartMessage) {
    log(s"Game starting!")
  }
  
  def onEnd(m: EndMessage) {
    val result = m.winnerTeamId match {
      case Some(t) if t == m.you.teamId => green"you win"
      case Some(t) => red"you lose"
      case None => cyan"game tied"
    }
    
    log(s"Game over, $result!")
    client.disconnect
  }
  
  def onEvents(m: EventsMessage) {
    val actions = ai.makeDecisions(m.roundId, m.events, m.you.bots, gameConfig)
    send(ActionsMessage(m.roundId, actions.groupBy(_.botId).values.map(_.head).toList))
  }
  
  private[tyckiting] def send(m: ClientMessage) =
    client.send(Json.stringify(Json.toJson(m)))
    
  private def log(m: String) =
    println(s"[${cyan"Client"}] $m")
}
