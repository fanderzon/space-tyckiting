(ns tyckiting.core
  (:require [gniazdo.core           :as ws]
            [clojure.data.json      :as json]
            [clojure.pprint         :as pp]
            [camel-snake-kebab.core :refer :all]
            [tyckiting.game         :as game]
            [tyckiting.ai           :as ai]
            ))

(defn quit
  "Exit the application when websocket connection is lost."
  [status description]
  (let [message (str "WS connection closed: " description)]
    (println message)
    (System/exit status)))

(def socket (atom nil))

(defn connect
  "Connect to the backend and link a message handler function.
  The connection is stored in an atom to resolve it from other functions."
  [url handler]
  (reset! socket (ws/connect url
                  :on-receive handler
                  :on-close quit)))

(defn send-message
  "Send data to the backend. Turns Clojure map into JSON with
  camelcase keywords."
  [data]
  (let [msg (json/write-str data :key-fn ->camelCaseString)]
    (println "=> Sending message:")
    (pp/pprint data)
    (ws/send-msg @socket msg)))

(defn join-game [team-name]
  (send-message {:type "join" :team-name team-name}))

(defn send-actions [actions round-id]
  (send-message {:type "actions"
                 :round-id round-id
                 :actions actions}))

(defn wrap-json-parser
  "Parse json to map and create Clojure style keywords before
  calling the handler."
  [handler]
  (fn [message]
    (-> message
      (json/read-str :key-fn ->kebab-case-keyword)
      (handler))))

(defmulti message-handler
  "Multi method handling incoming websocket messages.
  Dispatches on the :type of message "
  :type)

(defn message-handler*
  "Message handler wrapped with logging"
  [msg]
  (println "=> Message received:")
  (pp/pprint msg)
  (message-handler msg))

(defmethod message-handler :default
  [msg]
  (println "UNKNOWN MESSAGE --" msg))

(defmethod message-handler "connected"
  [{:keys [config team-id]}]
  (game/set-config  config)
  (game/set-team    team-id)
  (join-game        ai/team-name))

(defmethod message-handler "start"
  [{:keys [opponents you]}]
  (game/set-opponents opponents)
  (game/set-bots (:bots you)))

(defmethod message-handler "end"
  [{:keys [winner-team-id]}]
  (println
    (cond
      (= winner-team-id (:team-id @game/state)) "YOU WIN"
      (= winner-team-id nil)                    "TIE GAME"
      :else                                     "YOU LOSE")))

(defmethod message-handler "events"
  [{:keys [round-id] :as msg}]
  (-> (game/new-round msg)
    (send-actions round-id)))

(defn start
  "Start the client by connecting to the specified backend and
  providing a handler for incoming websocket messages."
  [{:keys [host port ai]}]
  (let [url (str "ws://" host ":" port)]
    (ai/register-ai ai)
    (connect url (-> message-handler*
                  (wrap-json-parser)))))
