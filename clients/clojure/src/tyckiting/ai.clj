(ns tyckiting.ai)

;; team name must be unique in a match, thus adding a random number
(def team-name (str "dummy team" (rand-int 9999)))

(def current-ai (atom nil))

(defn- require-ai
  "Tries to require the given namespace."
  [ns-symbol]
  (try
    (require ns-symbol)
    (catch java.io.FileNotFoundException e
      (println (.getMessage e))
      (println (str "\nERROR: The AI \"" ns-symbol "\" was not found.\n"
                 "Please check your namespaces.\n"
                 "The AI needs to use the namespace tyckiting.ai.<AI-NAME>\n"
                 "An example AI can be found in tyckiting.ai.random."))
      (System/exit 1))))

(defn- register-ai-function
  "Register the 'creat-actions' function of the given namespace to be used
   in the upcoming game."
  [ns-symbol]
  (require-ai ns-symbol)
  (when-not (->> (ns-resolve (find-ns ns-symbol) 'create-actions)
              (reset! current-ai))
    (do (println (str  "\nERROR: The AI Namespace \"" ns-symbol "\" "
                   "has no 'create-actions' function.\n"
                   "Please add it. "
                   "An example AI can be found in tyckiting.ai.random."))
        (System/exit 1))))

(defn register-ai
  "Registers the AI with the given name to be used during game to be joined.
   Exits the programm if the AI namespace cannot be found or does not have the
   'create-actions' function as en entry point."
  [ai-name]
  (let [ns-symbol (symbol (str "tyckiting.ai." ai-name))]
    (register-ai-function ns-symbol)))
