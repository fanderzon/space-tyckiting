{-# LANGUAGE OverloadedStrings #-}

module Tyckiting.Messages (
  -- * Client to Server messages
  -- ** Game start
  JoinMessage(..),
  -- ** Game loop
  MoveMessage(..),
  -- * Server to Client messages
  ConfMessage(..),
  StartMessage(..),
  -- ** Game loop
  ServerMessage(..)
  ) where

import Prelude        ()
import Prelude.Compat

import Data.Aeson    (FromJSON (..), ToJSON (..), withObject, Value (..), object, (.:),
                      (.=))
import Data.Text     (Text)

import Tyckiting.Action
import Tyckiting.AI
import Tyckiting.Event
import Tyckiting.Types

-- | Join message
--
-- > {
-- >   "type": "join",
-- >   "teamName": "Dummy 93"
-- > }
data JoinMessage = JoinMessage BotName
    deriving (Show)

instance ToJSON JoinMessage where
    toJSON (JoinMessage (BotName name))  = object
        [ "type" .= ("join" :: Text)
        , "teamName" .= name
        ]

instance FromJSON JoinMessage where
    parseJSON = withObject "JoinMessage" $ \v -> v .: "type" >>= dispatch v
      where
        dispatch v t | t == ("join" :: Text)  = parseJoin v
                     | otherwise              = fail "Invalid JoinMessage"
        parseJoin v = JoinMessage . BotName <$> v .: "name"

-- | Action message.
data MoveMessage = MoveMessage RoundNumber [Action]
  deriving (Show)

instance ToJSON MoveMessage where
    toJSON (MoveMessage roundN actions) = object
        [ "type" .= ("actions" :: Text)
        , "roundId" .= roundN
        , "actions" .= actions
        ]

-- | Connected message.
data ConfMessage = ConfMessage GameConfig
    deriving (Show)

instance ToJSON ConfMessage where
    toJSON (ConfMessage _info) =
        object ["type" .= ("connected" :: Text) ] -- TODO

instance FromJSON ConfMessage where
    parseJSON = withObject "ConfMessage" $ \v -> v .: "type" >>= dispatch v
      where
        dispatch v t | t == ("connected" :: Text)  = start v
                     | otherwise                   = fail "Invalid ConfMessage"
        start v  = ConfMessage <$> v .: "config"

-- | Game Start Message.
data StartMessage = StartMessage GameConfig
    deriving (Show)

instance FromJSON StartMessage where
    parseJSON = withObject "StartMessage" $ \v -> v .: "type" >>= dispatch v
      where
        dispatch v t | t == ("start" :: Text)  = start v
                     | otherwise               = fail "Invalid StartMessage"
        start v   = StartMessage <$> v .: "config"

-- | Server Messages
data ServerMessage = EventsMessage GameState [Event]
                   | GameOverMessage (Team WithHpAndPos) (Maybe TeamId)
    deriving (Show)

newtype MaybeTeamId = MaybeTeamId { getMaybeTeamId :: Maybe TeamId }

instance FromJSON MaybeTeamId where
    parseJSON = fmap MaybeTeamId . parseJSON

instance ToJSON ServerMessage where
    toJSON (EventsMessage gs es)              = object ["type" .= ("events" :: Text), "state" .= show gs, "events" .= show es]
    toJSON (GameOverMessage _you Nothing)     = object ["type" .= ("gameover" :: Text), "winnerTeamId" .= Null ]
    toJSON (GameOverMessage _you (Just tid))  = object ["type" .= ("gameover" :: Text), "winnerTeamId" .= tid ]

instance FromJSON ServerMessage where
    parseJSON = withObject "ServerMessage" $ \v -> v .: "type" >>= dispatch v
      where
        dispatch v t | t == ("events" :: Text)  = events v
                     | t == ("end" :: Text)     = gameover v
                     | otherwise                = fail "Invalid ServerMessage"
        events v   = EventsMessage
            <$> parseJSON (Object v)
            <*> v .: "events"
        gameover v = GameOverMessage
            <$> v .: "you"
            <*> (getMaybeTeamId <$> v .: "winnerTeamId")
