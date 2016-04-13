(defproject tyckiting "0.1.0-SNAPSHOT"
  :description "Space Tyckiting Clojure client"
  :url "http://futurice.com"
  :license {:name "Eclipse Public License"
            :url "http://www.eclipse.org/legal/epl-v10.html"}
  :dependencies [[org.clojure/clojure "1.6.0"]
                 [org.clojure/tools.cli "0.3.1"]      ; CLI helpers
                 [stylefruits/gniazdo "0.3.1"]        ; Websocket communication
                 [org.clojure/data.json "0.2.6"]      ; handling JSON
                 [prismatic/schema "0.4.0"]           ; data description and validation
                 [camel-snake-kebab "0.3.1"           ; JSON<>CLJ conversions for
                  :exclusions [org.clojure/clojure]]] ; different naming schemes
  :main tyckiting.cli)
