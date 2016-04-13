module Options (WebGame(..), Options(..), defaultMain) where

import Data.List.Split
import Data.String
import Options.Applicative.Extras as O
import Tyckiting.AI
import Tyckiting.Types

-- | Command line options.
data WebGame = WebGame String String String -- ^ User Pass BotName
    deriving (Eq, Ord, Show, Read)

data Options = Options String (Maybe Int) BotName (GameConfig -> IO AIAutomaton) (Maybe WebGame)

-- | Command line options parser definition.
webGameParser :: O.Parser WebGame
webGameParser = option
    (eitherReader parser)
    (hidden <> long "webgame" <> metavar "SPEC" <> help "Ask for game, format user:pass:botname")
  where
    parser s = case splitOn ":" s of
        [user,pass,botname] -> Right $ WebGame user pass botname
        _                   -> Left $ "incorrect webgame format -- " ++ s

options :: [(String, AI)] -> O.Parser Options
options ais = Options
    <$> strOption                 (long "host" <> short 'H' <> metavar "HOST" <> value "localhost" <> help "Host to connect to")
    <*> optional (option auto     (long "port" <> short 'P' <> metavar "PORT" <> help "Port to connect to"))
    <*> (fromString <$> strOption (long "name" <> short 'n' <> metavar "NAME" <> value "CurryBot" <> help "Bot's name"))
    <*> option (lookupReader ais) (long "ai"   <> short 'a' <> metavar "AI"   <> value defaultAi <> help "Select AI")
    <*> optional webGameParser
  where defaultAi = snd $ head ais

-- | Application entry point.
defaultMain :: [(String, AI)] -> (Options -> IO ()) -> IO ()
defaultMain ais run = O.execParser options' >>= run
  where
    options' = info (helper <*> options ais)
                    (fullDesc
                     <> progDesc "Destroy 'em all"
                     <> header "tyckiting-client - a base for your AI")
