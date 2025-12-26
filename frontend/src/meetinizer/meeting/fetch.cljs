(ns meetinizer.meeting.fetch
  (:require
    [meetinizer.the-state :refer [state-atom]]))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/meeting/" id))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)
                   (swap! state-atom assoc-in [:meeting id] {:title "Hello"})))))))

(defn fetch-whoami [id]
  (-> (js/fetch (str "/api/meeting/" id "/whoami"))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (prn "status" status)
                 (condp = status
                   403 (swap! state-atom assoc-in [:whoami id] :forbidden)
                   (do
                     (-> (.text the-result)
                         (.then (fn [body]
                                  (swap! state-atom assoc-in [:whoami id] body)
                                  ))
                         ))))))))

(defn login [id email]
  (-> (js/fetch (str "/api/meeting/" id "/request-login/" email))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)
                   (swap! state-atom assoc-in [:meeting id] :requested)))))))

(comment
  (fetch-meeting "123")
  (login "123" "kalle")
  (fetch-whoami "555")
  @state-atom
  )
