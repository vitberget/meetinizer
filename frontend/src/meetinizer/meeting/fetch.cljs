(ns meetinizer.meeting.fetch
  (:require [meetinizer.the-state :refer [state-atom]]))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/meeting/" id))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
               ; (println "got result" status)
                (condp = status
                  403 (swap! state-atom assoc-in [:meeting id] :forbidden)

                  (swap! state-atom assoc-in [:meeting id] {:title "Hello"})
                  )
                ) 
               ; (js/console.log the-result)
               ))

      ))

(comment
  (fetch-meeting "123")
  )
