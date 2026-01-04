(ns meetinizer.meeting.fetch
  (:require [meetinizer.the-state :refer [state-atom]]))

(defn update-meeting-state [id the-result]
  (let [status (.-status the-result)]
                 (condp = status
                   200 (-> (.json the-result)
                           (.then (fn [json]
                                    (let [data (js->clj json :keywordize-keys true)]
                                      (swap! state-atom assoc-in [:meeting id] data)))))

                   403 (swap! state-atom assoc-in [:meeting id] :forbidden)

                   (swap! state-atom assoc-in [:meeting id] :error))))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/meeting/" (js/encodeURIComponent  id)))
      (.then (fn [the-result] (update-meeting-state id the-result)))))

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

(defn logout [id]
  (-> (js/fetch (str "/api/meeting/" id "/logout"))
      (.then (fn[_] (swap! state-atom assoc-in [:meeting id] nil)))))

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

(defn register-name [meeting-name username]
  (-> (js/fetch (str "/api/meeting/" meeting-name "/register-name") 
                (clj->js {:method "POST" 
                          :headers {"Content-Type" "application/json"}
                          :body (.stringify js/JSON (clj->js {:name username}))}))
      (.then (fn [the-result] (update-meeting-state meeting-name the-result)))))

(defn add-vote [meeting-name vote]
  (-> (js/fetch (str "/api/meeting/" meeting-name "/vote/add") 
                (clj->js {:method "POST" 
                          :headers {"Content-Type" "application/json"}
                          :body (.stringify js/JSON (clj->js vote))}))
      (.then (fn [the-result] (update-meeting-state meeting-name the-result)))))

(defn rm-vote [meeting-name vote]
  (-> (js/fetch (str "/api/meeting/" meeting-name "/vote/rm") 
                (clj->js {:method "POST" 
                          :headers {"Content-Type" "application/json"}
                          :body (.stringify js/JSON (clj->js vote))}))
      (.then (fn [the-result] (update-meeting-state meeting-name the-result)))))

(defn meeting-sse [id]
  (let [sse (js/EventSource. (str "/api/meeting/" id "/sse"))]
    (swap! state-atom assoc-in [:sse id] sse)
    (set! (.-onmessage sse) (fn[event] 
                              (let [data (as-> event $
                                           (.-data $)
                                           (.parse js/JSON $)
                                           (js->clj $ {:keywordize-keys true}))]
                                (swap! state-atom assoc-in [:meeting id] data)
                                (prn "event" data))))
    (set! (.-onerror sse) (fn [error]
                            (prn "error sse" error)
                            (.close sse) 
                            (swap! state-atom update-in [:meeting] dissoc id)))))

(defn stop-sse [id]
  (.close (get-in @state-atom [:sse id]))
  (swap! state-atom update-in [:sse] dissoc id))

(comment
  (meeting-sse "alive")
  (register-name "alive" "Kenneth")
  (fetch-meeting "777")
  (login "123" "kalle")
  (fetch-whoami "777")
  @state-atom
  (swap! state-atom assoc :whoami nil)
  )
