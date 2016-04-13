require 'json'
require 'active_support/core_ext/hash/keys'
require File.expand_path('../game_config', __FILE__)
require File.expand_path('../bot_data', __FILE__)
require File.expand_path('../event', __FILE__)

class Message
  attr_accessor :type, :teamId, :config, :roundId, :bots_data, :events, :winnerTeamId

  def initialize(msg)
    @json = JSON.parse(msg).deep_symbolize_keys
    self.type = @json[:type]
    self.teamId = @json[:teamId]
    self.config = GameConfig.new @json[:config] if @json[:config]
    self.roundId = @json[:roundId]
    self.winnerTeamId = @json[:winnerTeamId]
    self.bots_data = @json[:you][:bots].map {|bot_data| BotData.new bot_data } if @json[:you]
    self.events = @json[:events].map {|event| Event.new event } if @json[:events]
  end

  def self.prepare(hash)
    JSON.generate(hash)
  end
end
