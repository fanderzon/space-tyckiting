module Ai

open Domain

val makeDecisions: UserState * GameConfig * GameState * Event list -> UserState * Action list
