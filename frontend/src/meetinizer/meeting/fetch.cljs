(ns meetinizer.meeting.fetch
  (:require [meetinizer.the-state :refer [state-atom]]))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/meeting/" id))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   200 (-> (.json the-result)
                           (.then (fn [json]
                                    (let [data (js->clj json :keywordize-keys true)]
                                      (swap! state-atom assoc-in [:meeting id] data)))))

                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)

                   (swap! state-atom assoc-in [:meeting id] :error)))))))

(defn fetch-whoami [id]
  (-> (js/fetch (str "/api/meeting/" id "/whoami"))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   200 (-> (.text the-result)
                           (.then (fn [body]
                                    (swap! state-atom assoc-in [:whoami id] body))))

                   403 (swap! state-atom assoc-in [:whoami id] :forbidden)

                   (swap! state-atom assoc-in [:whoami id] :error)))))))

(defn login [id email]
  (-> (js/fetch (str "/api/meeting/" id "/request-login/" email))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   ; TODO show seconds left
                   200 (-> (.text the-result) 
                           (.then (fn[text]
                                    (swap! state-atom assoc-in [:meeting id] {:requested text}))))

                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)

                   (swap! state-atom assoc-in [:meeting id] :error)))))))

(comment
  (fetch-meeting "777")
  (login "123" "kalle")
  (fetch-whoami "777")
  @state-atom
  (swap! state-atom assoc :whoami nil)
  )
