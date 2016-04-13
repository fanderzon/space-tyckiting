package tyckiting

import play.api.libs.json._

package object reads {
  implicit val rp = Json.reads[Position]
  implicit val rgc = Json.reads[GameConfig]
  implicit val rb = Json.reads[Bot]
  implicit val rt = Json.reads[Team]
  implicit val rbn = Json.reads[BotNoPosNoHp]
  implicit val rtn = Json.reads[TeamNoPosNoHp]
  
  implicit val re1  = Json.reads[HitEvent]
  implicit val re2  = Json.reads[DamagedEvent]
  implicit val re3  = Json.reads[DieEvent]
  implicit val re4  = Json.reads[SeeEvent]
  implicit val re5  = Json.reads[RadarEchoEvent]
  implicit val re6  = Json.reads[DetectedEvent]
  implicit val re7  = Json.reads[NoActionEvent]
  implicit val re8  = Json.reads[MoveEvent]
  implicit val re9  = Json.reads[SeeAsteroidEvent]
  implicit val re10 = Json.reads[UnknownEvent]
  
  implicit val re = Reads[Event](js => (js \ "event").validate[String].flatMap {
    case "hit"         => js.validate[HitEvent]
    case "damaged"     => js.validate[DamagedEvent]
    case "die"         => js.validate[DieEvent]
    case "see"         => js.validate[SeeEvent]
    case "radarEcho"   => js.validate[RadarEchoEvent]
    case "detected"    => js.validate[DetectedEvent]
    case "noaction"    => js.validate[NoActionEvent]
    case "move"        => js.validate[MoveEvent]
    case "seeAsteroid" => js.validate[SeeAsteroidEvent]
    case event         => js.validate[UnknownEvent]
  })
  
  implicit val rm1 = Json.reads[ConnectedMessage]
  implicit val rm2 = Json.reads[StartMessage]
  implicit val rm3 = Json.reads[EventsMessage]
  implicit val rm4 = Json.reads[EndMessage]
  
  implicit val rsm = Reads[ServerMessage](js => (js \ "type").validate[String].flatMap {
    case "connected" => js.validate[ConnectedMessage]
    case "start"     => js.validate[StartMessage]
    case "events"    => js.validate[EventsMessage]
    case "end"       => js.validate[EndMessage]
  })
}

package object writes {
  implicit val wp = Json.writes[Position]
  
  implicit val wm1 = Json.writes[MoveAction]
  implicit val wm2 = Json.writes[RadarAction]
  implicit val wm3 = Json.writes[CannonAction]
    
  implicit val wa = Writes[Action](_ match {
    case m: MoveAction => wm1.writes(m)
    case m: RadarAction => wm2.writes(m)
    case m: CannonAction => wm3.writes(m)
  })
  
  implicit val wm4 = Json.writes[JoinMessage]
  implicit val wm5 = Json.writes[ActionsMessage]
  
  implicit val wcm = Writes[ClientMessage](_ match {
    case m: JoinMessage => wm4.writes(m)
    case m: ActionsMessage => wm5.writes(m)
  })
}
