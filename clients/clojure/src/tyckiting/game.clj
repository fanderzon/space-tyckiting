(ns tyckiting.game
  (require [tyckiting.ai :as ai]))

;; put all the state into a single atom
(def state (atom {:bots      nil
                  :opponents nil
                  :team-id   nil
                  :config    nil}))

(defn- swap-state
  "Sets a new value for the given key in the state map."
  [key value]
  (swap! state assoc key value))

(defn set-config [config]
  (swap-state :config config))

(defn set-team [id]
  (swap-state :team-id id))

(defn set-opponents [opps]
  (swap-state :opponents opps))

(defn set-bots [bots]
  (swap-state :bots bots))

(defn update-bots [bots]
  (set-bots bots))

(defn new-round
  "Process a new game round."
  [{:keys [you current-round] :as round-data}]
  (update-bots (:bots you))
  ((deref ai/current-ai) (merge round-data @state)))
