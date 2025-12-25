(ns meetinizer.eventhandler 
  (:require
   [clojure.walk :as walk]
   [meetinizer.the-state :refer [state-atom]]))

(defn- enrich-action-from-event [{:replicant/keys [js-event node]} actions]
  (walk/postwalk
    (fn [x]
      (cond
        (keyword? x)
        (case x
          :event/target.value (-> js-event .-target .-value)
          :dom/node node
          x)
        :else x))
    actions))

(defn do-the-login []
  (prn "do-the-login")  
  )

(defn event-handler [{:replicant/keys [^js js-event] :as replicant-data} actions]
  (doseq [action actions]
    (prn "Triggered action" action)
    (let [enriched-action (->> action
                               (enrich-action-from-event replicant-data)
                               ; (enrich-action-from-state @!state)
                               )
          [action-name & args] enriched-action]
      (prn "Enriched action" enriched-action)
      (condp = action-name
        :db/assoc (apply swap! state-atom assoc args)
        :auth/login (do-the-login)
        
        
        )))
  ; (main-thing el @state-atom)
  )
