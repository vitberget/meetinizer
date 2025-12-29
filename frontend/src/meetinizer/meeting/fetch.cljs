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

(defn register-name [meeting-name meeting-uuid meeting-revision username]
  (-> (js/fetch (str "/api/meeting/" meeting-name "/register-name") 
                (clj->js {:method "POST" 
                          :headers {"Content-Type" "application/json"}
                          :body (.stringify js/JSON (clj->js {;:meeting_uuid meeting-uuid
                                                              ;:meeting_revision meeting-revision
                                                              :name username}))
                          }))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (prn status)
                 ; (condp = status
                 ;   ; TODO show seconds left
                 ;   200 (-> (.text the-result) 
                 ;           (.then (fn[text]
                 ;                    (swap! state-atom assoc :meeting-ids :requested ))))
                 ;
                 ;   403 (swap! state-atom assoc :meeting-ids :forbidden)
                 ;
                 ;   (swap! state-atom assoc :meeting-ids :error))
                 )))))

(comment
  (register-name "alive" "497eb28f-2f5a-4668-8275-22904646bfe5" "34ecf428-5ff8-42d2-ae37-051f384c4b10" "Kenneth")
  (fetch-meeting "777")
  (login "123" "kalle")
  (fetch-whoami "777")
  @state-atom
  (swap! state-atom assoc :whoami nil)
  )
