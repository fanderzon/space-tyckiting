{-# LANGUAGE OverloadedStrings #-}
module Tyckiting.Action (Action(..)) where

import Data.Aeson       (ToJSON (..), object, (.=), Value)
import Data.Aeson.Types (Pair)
import Data.Text        (Text)

import Tyckiting.Position
import Tyckiting.Types

data Action
    = MoveAction BotId Position
    | CannonAction BotId Position
    | RadarAction BotId Position
    deriving (Show, Read)

toJSONAction :: Text -> [Pair] -> Value
toJSONAction action pairs = object $ ("type" .= action) : pairs

instance ToJSON Action where
    toJSON (MoveAction bid pos)   = toJSONAction "move"   [ "botId" .= bid, "pos" .= pos ]
    toJSON (CannonAction bid pos) = toJSONAction "cannon" [ "botId" .= bid, "pos" .= pos ]
    toJSON (RadarAction bid pos)  = toJSONAction "radar"  [ "botId" .= bid, "pos" .= pos ]
