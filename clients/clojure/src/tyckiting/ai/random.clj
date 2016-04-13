(ns tyckiting.ai.random
  (:require [tyckiting.actions :as actions]))

(defn- rand-in-range
  "Returns a random number between min and max (inclusive)"
  [min max]
  (+ (rand-int (- (+ 1 max) min)) min))

(defn- random-move
  "Move bot randomly"
  [bot]
  (actions/move bot (rand-in-range -2 2) (rand-in-range -2 2)))

(defn- random-cannon
  "Shoot cannon randomly"
  [bot]
  (actions/cannon bot (rand-in-range -5 5) (rand-in-range -5 5)))

(defn- random-action
  "Pick a random, predefined action"
  [bot]
  (let [action (rand-nth [random-move random-cannon])]
    (action bot)))

(defn create-actions
  [game-state]
  (let [bots (get-in game-state [:you :bots])]
    (map random-action bots)))
