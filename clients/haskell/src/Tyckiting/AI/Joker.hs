{-# LANGUAGE FlexibleContexts #-}
module Tyckiting.AI.Joker (jokerAI) where

import Prelude        ()
import Prelude.Compat

import Control.Monad.Random (getRandomR, uniform)
import Control.Monad.RWS    (get, put)
import Data.Foldable        (minimumBy)
import Data.Function        (on)
import Data.List            (nub)
import Data.Maybe           (mapMaybe)

import Tyckiting.Action
import Tyckiting.AI
import Tyckiting.Event
import Tyckiting.Position
import Tyckiting.Types

-- | Our state consist of positions we didn't yet shooted at
type JokerState = [Position]

-- | Positions on the whole map, yet without borders
allPositions :: GameConfig -> JokerState
allPositions gameConfig =
    neighbours (cfgFieldRadius gameConfig - cfgCannon gameConfig) origo

-- | Initial state of our bot. Not strictly necessary with this AI
mkInitialState :: GameConfig -> IO JokerState
mkInitialState = return . allPositions

-- | Take the nth element from the list
takeNth :: Int -> [a] -> Maybe (a, [a])
takeNth n ls = case ys of
    (y:ys') -> Just (y, xs ++ ys')
    []      -> Nothing
  where (xs, ys) = splitAt n ls

-- | Get random position to shoot at
randomShotTarget :: NDAIMonad JokerState Position
randomShotTarget = do
    pos <- get
    case pos of
        []    -> (allPositions . gsGameConfig <$> askGameState) >>= randomShotTarget'
        (_:_) -> randomShotTarget' pos
  where
    randomShotTarget' pos = do
        n <- getRandomR (0, length pos - 1)
        case takeNth n pos of
            Nothing        -> return $ origo  -- Should never happen, but we want to stay total still
            Just (p, pos') -> put pos' >> return p

-- | Safer version, we avoid shoting close to our bots.
-- @Int@ parameter verifies we don't try too much.
saferRandomShotTarget :: [Position] -> Int -> NDAIMonad JokerState Position
saferRandomShotTarget _      0 = randomShotTarget
saferRandomShotTarget botPos n = do
    gameConfig <- gsGameConfig <$> askGameState
    pos <- randomShotTarget
    if minimumDistance botPos pos > cfgCannon gameConfig + cfgMove gameConfig
       then return pos
       else saferRandomShotTarget botPos (n - 1)

safeMinimumBy :: (a -> a -> Ordering) -> [a] -> Maybe a
safeMinimumBy _ []       = Nothing
safeMinimumBy f xs@(_:_) = Just $ minimumBy f xs

lookupClosest :: Position -> [Event] -> Maybe Position
lookupClosest to = safeMinimumBy (compare `on` distance to) . mapMaybe f
  where
    f (SeeEvent _ pos) = Just pos
    f _                = Nothing

minimumDistance :: [Position] -> Position -> Word
minimumDistance as b = minimum $ map (distance b) as

evade :: BotId -> GameConfig -> Position -> NDAIMonad JokerState Action
evade bid gameConfig pos =
    MoveAction bid <$> uniform possibleMoves
  where
    possibleMoves
        = nub
        . map (clamp (cfgFieldRadius gameConfig))
        . filter ((cfgMove gameConfig ==) . distance pos)
        . neighbours (cfgMove gameConfig)
        $ pos

randomShot :: BotId -> [Position] -> NDAIMonad JokerState Action
randomShot bid ownBotPos =
    CannonAction bid <$> saferRandomShotTarget ownBotPos 100

detected :: BotId -> [Event] -> Bool
detected bid es = any f es
  where f (DetectedEvent botId') | botId' == bid = True
        f _                                      = False

moveBot :: Bot WithHpAndPos -> NDAIMonad JokerState Action
moveBot bot = do
    gameConfig <- gsGameConfig <$> askGameState
    ownBotPos <- map botPosition .  teamBots . gsYourTeam <$> askGameState :: NDAIMonad JokerState [Position]
    es <- askEvents
    if detected (botId bot) es
        then evade (botId bot) gameConfig (botPosition bot)
        else let maybeClosestPos = lookupClosest (botPosition bot) es
              in case maybeClosestPos of
                  Nothing -> randomShot (botId bot) ownBotPos
                  Just closestPos -> if (distance closestPos (botPosition bot) <= cfgSee gameConfig)
                                         then evade (botId bot) gameConfig (botPosition bot)
                                         else return $ CannonAction (botId bot) closestPos

move :: NDAIMonad JokerState [Action]
move = do
  -- Ask for info
  ownBots <- teamBots . gsYourTeam <$> askGameState :: NDAIMonad JokerState [Bot WithHpAndPos]
  es <- askEvents
  -- Log
  tellShow ownBots
  tellShow es
  -- Actions
  as <- traverse moveBot ownBots
  tellShow as
  return as

jokerAI :: AI
jokerAI = mkNDAI move mkInitialState
