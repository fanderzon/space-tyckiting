{-# LANGUAGE OverloadedStrings #-}
module Tyckiting.Event (Event(..)) where

import Prelude        ()
import Prelude.Compat

import Control.Applicative ((<|>))
import Control.Monad       (mzero)
import Data.Aeson          (FromJSON (..), Value (..), Object, (.:))
import Data.Aeson.Types    (Parser)
import Data.Text           (Text)

import Tyckiting.Position
import Tyckiting.Types

data Event
    = UnknownEvent Text
    | HitEvent BotId BotId     -- ^ we them
    | DamagedEvent BotId Word  -- ^ target damage
    | DieEvent BotId           -- ^ some?
    | RadarEchoEvent Position  -- ^ we them position
    | DetectedEvent BotId      -- ^ we
    | NoActionEvent BotId      -- ^ we
    | MoveEvent BotId Position -- ^ we position
    | SeeEvent BotId Position  -- ^ them position
    | SeeAsteroid Position     -- ^ asteroid position
    | EndEvent
  deriving (Show, Read)

eventParseJSON :: Object -> Text -> Parser Event
eventParseJSON _ "end"         = pure EndEvent
eventParseJSON v "see"         = SeeEvent
    <$> v .: "botId"
    <*> v .: "pos"
eventParseJSON v "seeAsteroid" = SeeAsteroid <$> v .: "pos"
eventParseJSON v "radarEcho"   = RadarEchoEvent <$> v .: "pos"
eventParseJSON v "move"        = MoveEvent
    <$> v .: "botId"
    <*> v .: "pos"
eventParseJSON v "noaction"    = NoActionEvent <$> v .: "botId"
eventParseJSON v "detected"    = DetectedEvent <$> v .: "botId"
eventParseJSON v "die"         = DieEvent <$> v .: "botId"
eventParseJSON v "damaged"     = DamagedEvent
    <$> v .: "botId"
    <*> v .: "damage"
eventParseJSON v "hit"         = HitEvent
    <$> v .: "source"
    <*> v .: "botId"
eventParseJSON _ t             = pure $ UnknownEvent t

instance FromJSON Event where
    parseJSON (Object v)  = (v .: "event" >>= eventParseJSON v) <|> pure (UnknownEvent "-")
    parseJSON _     = mzero
