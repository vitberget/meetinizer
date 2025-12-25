(ns meetinizer.meeting.fetch
  (:require [meetinizer.the-state :refer [state-atom]]))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/meeting/" id))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)
                   (swap! state-atom assoc-in [:meeting id] {:title "Hello"})))))))

(defn login [id email]
  (-> (js/fetch (str "/api/login/request/" id "/" email))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)
                   (swap! state-atom assoc-in [:meeting id] :requested)))))))

(comment
  (fetch-meeting "123")
  (login "123" "kalle")
  )
