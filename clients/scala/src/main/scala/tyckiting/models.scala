package tyckiting

case class GameConfig(
  bots: Int, fieldRadius: Int, move: Int, startHp: Int, cannon: Int,
  radar: Int, see: Int, maxCount: Int, loopTime: Int)
  
case class Position(x: Int, y: Int)
case class Bot(botId: Int, name: String, hp: Int, alive: Boolean, pos: Position)
case class Team(teamId: Int, name: String, bots: List[Bot])
case class BotNoPosNoHp(botId: Int, name: String, alive: Boolean)
case class TeamNoPosNoHp(teamId: Int, name: String, bots: List[BotNoPosNoHp])

sealed trait Event
case class HitEvent(event: String, botId: Int, source: Int) extends Event
case class DamagedEvent(event: String, botId: Int, damage: Int) extends Event
case class DieEvent(event: String, botId: Int) extends Event
case class SeeEvent(event: String, botId: Int, source: Int, pos: Position) extends Event
case class RadarEchoEvent(event: String, botId: Int, source: Int, pos: Position) extends Event
case class DetectedEvent(event: String, botId: Int) extends Event
case class NoActionEvent(event: String, botId: Int) extends Event
case class MoveEvent(event: String, botId: Int, pos: Position) extends Event
case class SeeAsteroidEvent(event: String, pos: Position) extends Event
case class UnknownEvent(event: String) extends Event

sealed trait ServerMessage
case class ConnectedMessage(teamId: Int, config: GameConfig, `type`: String) extends ServerMessage
case class StartMessage(you: Team, otherTeams: List[TeamNoPosNoHp], `type`: String) extends ServerMessage
case class EventsMessage(roundId: Int, you: Team, otherTeams: List[TeamNoPosNoHp], events: List[Event], `type`: String) extends ServerMessage
case class EndMessage(you: Team, winnerTeamId: Option[Int], `type`: String) extends ServerMessage

sealed trait ClientMessage
case class JoinMessage(teamName: String, `type`: String = "join") extends ClientMessage
case class ActionsMessage(roundId: Int, actions: List[Action], `type`: String = "actions") extends ClientMessage

sealed trait Action { def botId: Int }
case class MoveAction(botId: Int, pos: Position, `type`: String = "move") extends Action
case class RadarAction(botId: Int, pos: Position, `type`: String = "radar") extends Action
case class CannonAction(botId: Int, pos: Position, `type`: String = "cannon") extends Action
