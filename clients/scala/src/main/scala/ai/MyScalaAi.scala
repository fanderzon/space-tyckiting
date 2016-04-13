package ai

import tyckiting._

object MyScalaAi extends Tyckiting {
  
  def teamName = "My Scala AI"
    
  def makeDecisions(
    roundId: Int,
    events: List[Event],
    bots: List[Bot],
    config: GameConfig) =
  {
    events.foreach(e => println(s"Event: $e"))
    bots.map(bot => bot.move(bot.pos.x + 1, bot.pos.y + 1))
  }
}
