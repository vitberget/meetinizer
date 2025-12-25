(ns meetinizer.eventhandler 
  (:require
   [clojure.walk :as walk]
   [meetinizer.meeting.fetch :refer [login]]
   [meetinizer.the-state :refer [get-path-parts state-atom]]))

(defn- enrich-action-from-state [state action]
  (walk/postwalk
   (fn [x]
     (cond
       (and (vector? x)
            (= :db/get (first x))) (get state (second x))
       :else x))
   action))

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

(defn do-the-login [email]
  (let [meeting-id (second (get-path-parts))]
    (swap! state-atom assoc-in [:meeting meeting-id] :requesting)
    (login meeting-id email)))

(defn event-handler [{:replicant/keys [^js js-event] :as replicant-data} actions]
  (doseq [action actions]
    (prn "Triggered action" action)
    (let [enriched-action (->> action
                               (enrich-action-from-event replicant-data)
                               (enrich-action-from-state @state-atom))
          [action-name & args] enriched-action]
      (prn "Enriched action" enriched-action)
      (condp = action-name
        :db/assoc (apply swap! state-atom assoc args)
        :auth/login (apply do-the-login args)


        )))
  ; (main-thing el @state-atom)
  )

(comment
  @state-atom
  )
