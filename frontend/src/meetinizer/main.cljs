(ns meetinizer.main
  (:require [clojure.string :as s] 
            [clojure.walk :as walk]
            [replicant.dom :as r]
            [meetinizer.meeting.render :as mr]
            [meetinizer.the-state :refer [state-atom]]))

(defn status-404 [] [:div [:h1 "404"]])

(defn main-thing [el state]
  (let [path js/window.location.pathname
        path-parts (->> (s/split path "/")
                        (drop 1))
        state (-> state 
                  (assoc :path path)
                  (assoc :path-parts path-parts))]
    (condp = (first path-parts)
      "meet" (r/render el (mr/render-meeting state))

      :else (r/render el (status-404)) )))

(defonce el (js/document.getElementById "app"))

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

(defn event-handler [{:replicant/keys [^js js-event] :as replicant-data} actions]
  (doseq [action actions]
    (prn "Triggered action" action)
    (let [enriched-action (->> action
                               (enrich-action-from-event replicant-data)
                               ; (enrich-action-from-state @!state)
                               )
          [action-name & args] enriched-action]
      (prn "Enriched action" enriched-action)))
  (main-thing el @state-atom))

(defn ^{:dev/after-load true :export true} main! []
  (println "main!")
  (add-watch state-atom ::render 
             (fn [_ _ _ state]
               (main-thing el state)))
  (main-thing el @state-atom))

(defn ^:export init! []
  ; (inspector/inspect "App state" !state)
  (println "init!")
  (r/set-dispatch! event-handler)
  (main!))
