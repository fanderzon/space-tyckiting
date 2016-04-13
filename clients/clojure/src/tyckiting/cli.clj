(ns tyckiting.cli
  (:require [clojure.tools.cli :refer [parse-opts]]
            [tyckiting.core    :as core])
  (:gen-class))

(def cli-options
  [["-H" "--host HOST"    "Host to connect to"]
   ["-P" "--port PORT"    "Port to connect to"]
   ["-a" "--ai AI"        "Select AI"]])

(defn missing-required?
  [opts]
  (not-every? opts #{:host :port :ai}))

(defn usage
  "Return the CLI usage information"
  [options-summary]
  (->> ["tyckiting-client - a base for your AI"
        "Usage: lein run [OPTIONS]"
        "  Destroy 'em all"
        ""
        "Mandatory parameters:"
        options-summary]
    (clojure.string/join \newline)))

(defn error-msg [errors]
  (str "The following errors occurred while parsing your command:\n\n"
    (clojure.string/join \newline errors)))

(defn exit [status msg]
  (println msg)
  (System/exit status))

(defn -main
  "Main entry point of the client, processing the command line options
  and passing them to start the client."
  [& args]
  (let [{:keys [options arguments errors summary]} (parse-opts args cli-options)]
    (cond
      (or (:help options)
        (= (count options) 0)
        (missing-required? options))
      (exit 0 (usage summary))
      errors (exit 1 (error-msg errors)))
    (core/start options)))
