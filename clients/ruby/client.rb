require 'optparse'
require 'colorize'
require 'websocket-eventmachine-client'
require File.expand_path('../lib/option_parser',  __FILE__)
require File.expand_path('../lib/message',  __FILE__)

options = OptionParserTykitys.parse(ARGV)

require File.expand_path('../ai/' + options.ai, __FILE__)
ai = AI.new

class Client
  attr_accessor :options, :ai, :ws, :was_connected, :has_ended

  def onconnected(message)
    puts "Connected to game server with team id: #{message.teamId.to_s.colorize(:light_blue)}."
    @ai.join(message.teamId, message.config)
    @ws.send Message.prepare(type: 'join', teamName: @options.name)
  end

  def onstart(message)
    @ai.start(message.bots_data)
    puts "Game started. Playing as: #{options.name.colorize(:light_blue)}."
  end

  def onend(message)
    result = if (!message.winnerTeamId)
      "Nobody won.".colorize(:light_blue)
    elsif (message.winnerTeamId == @ai.teamId)
      "We won.".colorize(:green)
    else
      "Enemy won.".colorize(:red)
    end

    puts "Game ended. "  + result
    @ws.close
    EM.stop
  end

  def onevents(message, msg)
    puts "Round #{message.roundId.to_s.colorize(:green)}."

    message.events.each do |event|
      pos = event.pos.to_s.colorize(:light_blue)
      botId = event.botId.to_s.colorize(:light_blue)

      case event.event
      when "die"
        puts "Bot #{botId} died."
      when "damaged"
        puts "Bot #{botId} was hit by enemy bot with damage #{event.damage.to_s.colorize(:light_blue)}."
      when "hit"
        puts "Bot #{event.source.to_s.colorize(:light_blue)} hit enemy bot #{botId}."
      when "detected"
        puts "Bot #{botId} detected at #{pos}."
      when "see"
        puts "Bot #{event.source.to_s.colorize(:light_blue)} detected enemy bot #{botId} at #{pos}."
      end
    end if message.events

    puts "Received message: #{msg.colorize(:light_black)}." if @options.verbose
    actions = @ai.move(message.bots_data, message.events)
    puts "Sending message: #{actions.to_s.colorize(:light_black)}." if @options.verbose
    @ws.send Message.prepare(type: 'actions', roundId: message.roundId, actions: actions)
  end

  def run(options, ai)
    self.options = options
    self.ai = ai
    self.was_connected = false
    self.has_ended = false

    uri = "ws://#{@options.host}:#{@options.port}"

    EM.run do
      puts "Connecting to: #{@uri.colorize(:light_blue)}" if @options.verbose
      self.ws = WebSocket::EventMachine::Client.connect(:uri => uri)

      @ws.onopen do
        puts "Connected" if @options.verbose
        self.was_connected = true
      end

      @ws.onmessage do |msg, type|
        message = Message.new(msg)

        case message.type
        when "connected"
          onconnected(message)
        when "start"
          onstart(message)
        when "end"
          onend(message)
          self.has_ended = true
        when "events"
          onevents(message, msg)
        else
          puts "Received message: #{msg.colorize(:light_black)}" if @options.verbose
          puts "Unknown message"
        end
      end

      @ws.onclose do |code, reason|
        if !was_connected
          puts "Looks like there is no server at: #{uri.colorize(:light_blue)}"
        elsif was_connected && !has_ended
          puts "Looks like the server died."
        end

        puts "Disconnected with status code: #{code.to_s.colorize(:red)} (#{reason})." if @options.verbose
        EM.stop
      end

    end
  end
end

Client.new.run(options, ai)
