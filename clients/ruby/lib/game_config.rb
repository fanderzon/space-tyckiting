class GameConfig
  attr_accessor :bots, :fieldRadius, :move, :startHp, :cannon, :radar, :see, :maxCount, :loopTime

  def initialize(hash)
    self.bots = hash[:bots]
    self.fieldRadius = hash[:fieldRadius]
    self.move = hash[:move]
    self.startHp = hash[:startHp]
    self.cannon = hash[:cannon]
    self.radar = hash[:radar]
    self.see = hash[:see]
    self.maxCount = hash[:maxCount]
    self.loopTime = hash[:loopTime]
  end
end
