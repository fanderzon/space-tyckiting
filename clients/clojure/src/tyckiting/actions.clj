(ns tyckiting.actions
  (:require [schema.core :as s]))

(def Action-Types
  (s/enum "move" "cannon" "radar"))

(def Action
  "A schema for bot actions"
  {:bot-id s/Int
   :type Action-Types
   :pos {:x s/Int
         :y s/Int}})

(s/defn
  ^:always-validate
  action :- Action
 "Produce data for an action to be sent to the server."
  [bot-id :- s/Int
   type   :- Action-Types
   x      :- s/Int
   y      :- s/Int]
  {:bot-id bot-id
   :type type
   :pos {:x x :y y}})

(defn move [bot dx dy]
  (let [id (:bot-id bot)
        x  (get-in bot [:pos :x])
        y  (get-in bot [:pos :y])]
    (action id "move" (+ x dx) (+ y dy))))

(defn cannon [bot x y]
  (let [id (:bot-id bot)]
    (action id "cannon" x y)))

(defn radar [bot x y]
  (let [id (:bot-id bot)]
    (action id "radar" x y)))
