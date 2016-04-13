{-# LANGUAGE OverloadedStrings #-}
module Messages where

import Prelude ()
import Prelude.Compat

import Data.Aeson

data NewGameRequest = NewGameRequest [String]
    deriving (Eq, Ord, Show, Read)

instance ToJSON NewGameRequest where
    toJSON (NewGameRequest bots) = object [ "bots" .= bots ]

data NewGameResponse = NewGameResponse Int String
    deriving (Eq, Ord, Show, Read)

instance FromJSON NewGameResponse where
    parseJSON = withObject "NewGameResponse" $ \v -> NewGameResponse
        <$> v .: "port"
        <*> v .: "host"

instance ToJSON NewGameResponse where
    toJSON (NewGameResponse port host) =
        object [ "port" .= port, "host" .= host ]
