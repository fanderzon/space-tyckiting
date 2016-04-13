module Tyckiting.AI.Timid (timidAI) where

import Prelude        ()
import Prelude.Compat

import Control.Monad.Random (uniform)
import Data.List            (nub)
import Data.Monoid          (First (..))

import Tyckiting.Action
import Tyckiting.AI
import Tyckiting.Event
import Tyckiting.Position
import Tyckiting.Types

-- Timid doesn't have any memory.
type TimidState = ()

-- And it doesn't do any precalc.
mkInitialState :: GameConfig -> IO TimidState
mkInitialState _ = return ()

-- Randomise where we could move.
moveBot :: Bot WithHpAndPos -> NDAIMonad TimidState Action
moveBot bot = do
    gameConfig <- gsGameConfig <$> askGameState
    MoveAction (botId bot) <$> uniform (possibleMoves gameConfig)
  where
    pos = botPosition bot
    possibleMoves gameConfig
        = nub
        . map (clamp (cfgFieldRadius gameConfig))
        . neighbours (cfgMove gameConfig)
        $ pos


cannonAt :: Position -> Bot w -> NDAIMonad TimidState Action
cannonAt pos bot = return $ CannonAction (botId bot) pos

-- | Return the first `Just` value predicate returns.
lookupFirst :: Foldable f => (a -> Maybe b) -> f a -> Maybe b
lookupFirst p = getFirst . foldMap (First . p)

seeAnyone :: [Event] -> Maybe Position
seeAnyone = lookupFirst p
  where
    p (SeeEvent _ pos) = Just pos
    p _                = Nothing

-- Actual move function
move :: NDAIMonad TimidState [Action]
move = do
    -- Ask for info
    ownBots <- teamBots . gsYourTeam <$> askGameState :: NDAIMonad TimidState [Bot WithHpAndPos]
    es <- askEvents
    -- Log
    tellShow ownBots
    tellShow es
    -- Cannon if we see anyone
    as <- case seeAnyone es of
        Just pos -> traverse (cannonAt pos) ownBots
        Nothing  -> traverse moveBot ownBots
    tellShow as
    return as

-- | Timid doesn't do much. It just moves around.
timidAI :: AI
timidAI = mkNDAI move mkInitialState
