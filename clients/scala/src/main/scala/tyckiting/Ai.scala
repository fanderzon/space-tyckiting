package tyckiting

trait Ai {
  def teamName: String
  def makeDecisions(
    roundId: Int,
    events: List[Event],
    bots: List[Bot],
    config: GameConfig): List[Action]
  
  implicit class BotMethods(bot: Bot) {
    def move(x: Int, y: Int) = MoveAction(bot.botId, Position(x, y))
    def radar(x: Int, y: Int) = RadarAction(bot.botId, Position(x, y))
    def cannon(x: Int, y: Int) = CannonAction(bot.botId, Position(x, y))
  }
}
