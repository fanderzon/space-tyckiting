{-# LANGUAGE OverloadedStrings #-}

module Tyckiting.Types where

import Prelude        ()
import Prelude.Compat

import Data.Aeson       (FromJSON (..), Object, ToJSON (..), withObject, (.:))
import Data.Aeson.Types (Parser)
import Data.String      (IsString (..))
import Data.Text        (Text)

import Tyckiting.Position

newtype BotId = BotId { getBotId :: Word }
    deriving (Eq, Ord, Show, Read)

instance ToJSON BotId where
    toJSON = toJSON . getBotId

instance FromJSON BotId where
    parseJSON v = BotId <$> parseJSON v

newtype TeamId = TeamId { getTeamId :: Word }
    deriving (Eq, Ord, Show, Read)

instance ToJSON TeamId where
    toJSON = toJSON . getTeamId

instance FromJSON TeamId where
    parseJSON v = TeamId <$> parseJSON v

newtype BotName = BotName Text
    deriving (Eq, Ord, Show, Read)

instance IsString BotName where
    fromString = BotName . fromString

instance FromJSON BotName where
    parseJSON v = BotName <$> parseJSON v

newtype RoundNumber = RoundNumber { getRoundNumber :: Int }
    deriving (Eq, Ord, Show, Read)

instance FromJSON RoundNumber where
    parseJSON v = RoundNumber <$> parseJSON v

instance ToJSON RoundNumber where
    toJSON = toJSON . getRoundNumber

data WithHpAndPos = WithHpAndPos Int Position
    deriving (Eq, Ord, Show, Read)

data WithNoExtra  = WithNoExtra
    deriving (Eq, Ord, Show, Read)

class BotExtra w where
    fromObject :: Object -> Parser w

instance BotExtra WithHpAndPos where
    fromObject v = WithHpAndPos
        <$> v .: "hp"
        <*> v .: "pos"

instance BotExtra WithNoExtra where
    fromObject _ = pure WithNoExtra

data Bot w = Bot
    { botId     :: BotId
    , botName   :: Text
    , botTeamId :: TeamId
    , botAlive  :: Bool
    , botExtra  :: w
    }
    deriving (Eq, Ord, Show, Read)

botPosition :: Bot WithHpAndPos -> Position
botPosition = extraPos . botExtra
  where extraPos (WithHpAndPos _ pos) = pos

instance BotExtra w => FromJSON (Bot w) where
    parseJSON = withObject "Bot"$ \v -> Bot
        <$> v .: "botId"
        <*> v .: "name"
        <*> v .: "teamId"
        <*> v .: "alive"
        <*> fromObject v

data Team w = Team
    { teamName :: Text
    , teamId   :: TeamId
    , teamBots :: [Bot w]
    }
    deriving (Eq, Ord, Show, Read)

instance BotExtra w => FromJSON (Team w) where
    parseJSON = withObject "Team" $ \v -> Team
        <$> v .: "name"
        <*> v .: "teamId"
        <*> v .: "bots"
