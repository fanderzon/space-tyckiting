require 'ostruct'

class OptionParserTykitys

  def self.parse(args)
    options = OpenStruct.new
    options.host = 'localhost'
    options.port = 3000
    options.verbose = false
    options.name = 'bot'
    options.ai = 'dummy'

    opt_parser = OptionParser.new do |opts|
      opts.banner = "Usage: main.rb [options]"

      opts.separator ""
      opts.separator "Destroy 'em all"
      opts.separator ""

      opts.separator "Specific options:"

      opts.on_tail("-h", "--help", "Show this message") do
        puts opts
        exit
      end

      opts.on("-H", "--host [HOST]", "Host to connect to") do |host|
        options.host = host
      end

      opts.on("-P", "--port [PORT]", OptionParser::DecimalInteger, "Port to connect to") do |port|
        options.port = port
      end

      opts.on("-n", "--name [NAME]", "Bot's name") do |name|
        options.name = name
      end

      opts.on("-a", "--ai [AI]", "AI package") do |ai|
        options.ai = ai
      end

      opts.on("-v", "--verbose", "Run verbosely") do |verbose|
        options.verbose = verbose
      end
    end

    opt_parser.parse!(args)
    options
  end
end
