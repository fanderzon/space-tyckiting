{-# LANGUAGE OverloadedStrings #-}
module Main (main) where

import Control.Concurrent (threadDelay)
import Control.Lens
import Control.Monad
import Data.Aeson         (FromJSON, ToJSON, eitherDecode, encode, toJSON)
import Data.Aeson.Lens
import Data.Maybe         (fromMaybe)
import Data.String        (fromString)
import Network.WebSockets
import Network.Wreq       hiding (Options)
import Prelude            as P

import Console
import Messages
import Options

import Tyckiting.AI
import Tyckiting.AI.Joker
import Tyckiting.AI.Timid
import Tyckiting.Messages
import Tyckiting.Types

-- | AIs
ais :: [(String, AI)]
ais = [("timid", timidAI)
      ,("joker", jokerAI)
      ]

-- | Receive the message.
receiveMessage :: FromJSON a => Connection -> IO (Either String a)
receiveMessage conn = do
  datum <- receiveData conn
  consoleLogBS "RECV" Magenta "<-" (Just Black) datum
  return $ eitherDecode datum

-- | Send the message.
sendMessage :: ToJSON a => Connection -> a -> IO ()
sendMessage conn x = do
  let datum = encode x
  consoleLogBS "RECV" Cyan "->" (Just Black) datum
  sendTextData conn datum

-- | Log user string.
userLog :: String -> IO ()
userLog = consoleLogString "INFO" Yellow "--" Nothing

-- | Wait for CONF message, send JOIN message, and proceed to s3.
s1 :: BotName -> AI -> Connection -> IO ()
s1 wishedTeamName initialAi conn = do
  message <- receiveMessage conn
  case message of
    Right (ConfMessage cfg) -> print cfg >> sendMessage conn joinMessage >> s3 initialAi conn
    Left err          -> P.putStrLn $ "ERROR! expecting CONF message -- " ++ show err -- will die
  where joinMessage = JoinMessage wishedTeamName

-- | Wait for START message, and proceed into game loop.
s3 :: AI -> Connection -> IO ()
s3 initialAi conn = do
  message <- receiveMessage conn
  case message of
    Right (StartMessage config) -> initialAi config >>= gameLoop conn
    Left err           -> P.putStrLn $ "ERROR! expecting START message -- " ++ show err

-- | Game loop: Wait for EVENTS or GAMEOVER, reply with MOVE if EVENTS.
gameLoop :: Connection -> AIAutomaton -> IO ()
gameLoop conn ai = do
  message <- receiveMessage conn
  case message of
    Right (GameOverMessage _ Nothing)       -> consoleLogString "TIE " Yellow "--" Nothing ""
    Right (GameOverMessage you (Just tid))
      | teamId you == tid                   -> consoleLogString "WIN " Green "!!" Nothing ""
      | otherwise                           -> consoleLogString "LOSE" Red   "!!" Nothing ""
    Right (EventsMessage gs es)         -> let (move, logs, ai') = runArrowAI ai (gs, es)
                                           in do mapM_ userLog logs
                                                 sendMessage conn (MoveMessage (gsRoundNumber gs) move) >> gameLoop conn ai'
    Left err                            -> P.putStrLn $ "ERROR! expecting EVENTS or GAMEOVER message -- " ++ show err

-- | Game entry point
game :: BotName -> AI -> Connection -> IO ()
game = s1

webSocketPath :: String
webSocketPath = "/"

webgameRequestPort :: Maybe Int -> Int
webgameRequestPort = fromMaybe 80

gamePort :: Maybe Int -> Int
gamePort = fromMaybe 3000

-- | Strongly typed `main`
run :: Options -> IO ()
run (Options h p wishedTeamName ai (Just (WebGame user pass botname))) = do
  let opts = defaults & auth ?~ basicAuth (fromString user) (fromString pass)
  r <- postWith opts ("http://" ++ h ++ ":" ++ show (webgameRequestPort p) ++ "/api/new") (toJSON $ NewGameRequest [botname])
  let maybePort = r ^? responseBody . _JSON
  case maybePort of
    Nothing                     -> fail "error in response"
    Just (NewGameResponse port host) -> do
      P.putStrLn $ "Created game at port " ++ show port
      P.putStrLn $ "Spectate at: http://" ++ host ++ ":" ++ show port
      threadDelay $ 5 * 1000000 -- so we have time to copy paste the spectator url
      runClient host port webSocketPath (game wishedTeamName ai)
run (Options host port wishedTeamName ai Nothing) =
  runClient host (gamePort port) webSocketPath (game wishedTeamName ai)

main :: IO ()
main = defaultMain ais run
