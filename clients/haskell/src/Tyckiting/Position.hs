{-# LANGUAGE OverloadedStrings #-}
module Tyckiting.Position (
    Position(..),
    Distance,
    distance,
    clamp,
    neighbours,
    origo,
    ) where

import Prelude        ()
import Prelude.Compat

import Data.Aeson (FromJSON (..), ToJSON (..), object, withObject, (.:), (.=))

-- | Hexagonal coordinate in axial format
--
-- @
-- x + y + z = 0 => z = - x - y
-- @
data Position = Position { posX :: Int, posY :: Int }
    deriving (Eq, Ord, Show, Read)

type Distance = Word

distance :: Position -> Position -> Distance
distance (Position ax ay) (Position bx by) = fromIntegral $ max (abs $ ax - bx) $ max (abs $ ay - by) (abs $ az - bz)
  where
    az = - ax - ay
    bz = - bx - by

towardsZero :: Int -> Int
towardsZero n = case compare n 0 of
    EQ -> 0
    LT -> n + 1
    GT -> n - 1

-- | Clamping is moving towards origo, until distance is less than clamping parameter
clamp :: Distance -> Position -> Position
clamp fieldRadius pos@(Position x y)
    | distance origo pos <= fieldRadius = pos
    | abs x >= abs y                    = clamp fieldRadius $ Position (towardsZero x) y
    | otherwise                         = clamp fieldRadius $ Position x (towardsZero y)

neighbours :: Distance -> Position -> [Position]
neighbours d pos@(Position x y) =
    [ pos'
    | x' <- [x-d'..x+d']
    , y' <- [y-d'..y+d']
    , not (x == x' && y == y')
    , let pos' = Position x' y'
    , distance pos pos' <= d
    ]
  where
    d' = fromIntegral d :: Int

instance FromJSON Position where
    parseJSON = withObject "Position" $ \v -> Position
        <$> v .: "x"
        <*> v .: "y"

instance ToJSON Position where
    toJSON (Position x y) = object [ "x" .= x , "y" .= y ]

origo :: Position
origo = Position 0 0
