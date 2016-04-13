{-# LANGUAGE FlexibleContexts  #-}
{-# LANGUAGE OverloadedStrings #-}
module Tyckiting.AI where

import Prelude        ()
import Prelude.Compat

import Control.Arrow.Transformer.Automaton

import Control.Monad.Random (RandT, runRandT)
import Control.Monad.RWS    (MonadReader (..), MonadWriter (..), RWS, runRWS)
import Data.Aeson           (FromJSON (..), withObject, (.:))
import System.Random.TF     (TFGen, mkSeedUnix, seedTFGen)

import Tyckiting.Action
import Tyckiting.Event
import Tyckiting.Position
import Tyckiting.Types

data GameConfig = GameConfig
    { cfgBotCount    :: Word
    , cfgFieldRadius :: Distance
    , cfgMove        :: Word
    , cfgStartHp     :: Word
    , cfgCannon      :: Word
    , cfgRadar       :: Word
    , cfgSee         :: Word
    , cfgMaxRound    :: RoundNumber
    , cfgLoopTime    :: Word
    }
    deriving (Show, Read)

instance FromJSON GameConfig where
    parseJSON = withObject "GameConfig object" $ \v -> GameConfig
        <$> v .: "bots"
        <*> v .: "fieldRadius"
        <*> v .: "move"
        <*> v .: "startHp"
        <*> v .: "cannon"
        <*> v .: "radar"
        <*> v .: "see"
        <*> (RoundNumber <$> v .: "maxCount")
        <*> v .: "loopTime"

-- | Game state. TBD
data GameState = GameState
    { gsRoundNumber :: RoundNumber
    , gsGameConfig  :: GameConfig
    , gsYourTeam    :: Team WithHpAndPos
    , gsOtherTeams  :: [Team WithNoExtra]
    }
    deriving (Show)

instance FromJSON GameState where
    parseJSON = withObject "GameState object" $ \v -> GameState
        <$> v .: "roundId"
        <*> v .: "config"
        <*> v .: "you"
        <*> v .: "otherTeams"

-- | Immutable (per-turn) context.
type GameContext = (GameState, [Event])

-- | AI type is @(GameState, [Event], UserState) -> (Move, UserState, [String])@.
-- Yet everything is hidden into monad transformer stack
type AIMonad us = RWS (GameState, [Event]) [String] us

runAIMonad :: AIMonad us a -> GameContext -> us -> (a, us, [String])
runAIMonad = runRWS

type NDAIMonad us = RandT TFGen (AIMonad us)

runNDAIMonad :: NDAIMonad us a -> GameContext -> us -> TFGen -> (a, us, [String], TFGen)
runNDAIMonad m r s g = sliceIn $ runRWS (runRandT m g) r s
  where sliceIn ((a, g'), s', w') = (a, s', w', g')

-- | Ask for this turn state.
--
-- @
-- askGameState :: AIMonad us GameState
-- askGameState :: NDAIMonad us GameState
-- @
askGameState :: (Functor m, MonadReader GameContext m) => m GameState
askGameState = fst <$> ask

-- | Aks for the game config
--
-- @
-- askGameConfig :: AIMonad us GameConfig
-- askGameConfig :: NDAIMonad us GameConfig
-- @
askGameConfig :: (Functor m, MonadReader GameContext m) => m GameConfig
askGameConfig = gsGameConfig <$> askGameState

-- | Ask for this turn's events.
--
-- @
-- askEvents :: AIMonad us [Event]
-- askEvents :: NDAIMonad us [Event]
-- @
askEvents :: (Functor m, MonadReader GameContext m) => m [Event]
askEvents = snd <$> ask

-- | Add a string to be logged by client.
--
-- @
-- tellLog :: String -> AIMonad us ()
-- tellLog :: String -> NDAIMonad us ()
-- @
tellLog :: MonadWriter [String] m => String -> m ()
tellLog str = tell [str]

tellShow :: (MonadWriter [String] m, Show a) => a -> m ()
tellShow = tellLog . show

type AI = GameConfig -> IO AIAutomaton

type Response = [Action]

mkAI :: AIMonad us Response -> (GameConfig -> IO us) -> AI
mkAI aimonad mkInitialState gameConfig =
    aiToArrowAI aimonad `fmap` mkInitialState gameConfig

mkNDAI :: NDAIMonad us Response -> (GameConfig -> IO us) -> AI
mkNDAI aimonad mkInitialState gameConfig =
    ndaiToArrowAI aimonad <$> g <*> mkInitialState gameConfig
  where g = seedTFGen <$> mkSeedUnix

-- | AI type with UserState hidden.
type AIAutomaton = Automaton (->) GameContext (Response, [String])

runArrowAI :: AIAutomaton -> GameContext -> (Response, [String], AIAutomaton)
runArrowAI (Automaton aut) ctx = sliceIn $ aut ctx
  where sliceIn ((a, b), c) = (a, b, c)

-- | Transform `AIMonad` to non-existential function (i.e. hide user state from the type).
aiToArrowAI :: AIMonad us Response -> us -> AIAutomaton
aiToArrowAI ai initialState = Automaton $ auto initialState
  where
    auto us ctx =
        let (move, us', logs) = runAIMonad ai ctx us
        in ((move, logs), Automaton $ auto us')

-- | Transform `NDAIMonad` to non-existential function.
ndaiToArrowAI :: NDAIMonad us Response -> TFGen -> us -> AIAutomaton
ndaiToArrowAI ai initialGen initialState =
    Automaton $ auto initialGen initialState
  where
      auto g us ctx =
          let (move, us', logs, g') = runNDAIMonad ai ctx us g
          in ((move, logs), Automaton $ auto g' us')
