(ns meetinizer.admin.fetch
  (:require
    [meetinizer.the-state :refer [state-atom]]))

(defn update-meeting-state [id the-result]
  (let [status (.-status the-result)]
    (prn "admin update-meeting-state" id status)
    (condp = status
      200 (-> (.json the-result)
              (.then (fn [json]
                       (let [data (js->clj json :keywordize-keys true)]
                         (swap! state-atom assoc-in [:admin :meeting id] data)))))

      403 (swap! state-atom assoc-in [:admin :meeting id] :forbidden)

      (swap! state-atom assoc-in [:admin :meeting id] :error))))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/admin/meeting/" id))
      (.then (fn [the-result] (update-meeting-state id the-result)))))


(defn fetch-meeting-list []
  (-> (js/fetch "/api/admin/meetings/list")
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   200 (-> (.json the-result)
                           (.then (fn [json]
                                    (let [data (js->clj json :keywordize-keys true)]
                                      (swap! state-atom assoc-in [:meeting-ids] data)))))

                   403 (swap! state-atom assoc-in [:meeting-ids] :forbidden)

                   (swap! state-atom assoc-in [:meeting-ids] :error)))))))

(defn create-meeting [meeting-name]
  (-> (js/fetch (str "/api/admin/meeting/" meeting-name "/create"))
      (.then (fn [_]
               (fetch-meeting-list)))))

(defn add-slot [id start end]
  (let [start (-> start (js/Date.) (.toISOString))
        end (-> end (js/Date.) (.toISOString))]
    (-> (js/fetch (str "/api/admin/meeting/" id "/slot/add")
                  (clj->js {:method "POST"
                            :headers {"Content-Type" "application/json"}
                            :body (.stringify js/JSON (clj->js {:start start :end end}))}))
        (.then (fn [the-result] (update-meeting-state id the-result))))))

(defn update-comment [id comment-text]
  (-> (js/fetch (str "/api/admin/meeting/" id "/comment")
                (clj->js {:method "POST"
                          :body comment-text}))
      (.then (fn [the-result] (update-meeting-state id the-result)))))

(defn rm-slot [id slot]
  (prn "rm-slot" id slot)
  (-> (js/fetch (str "/api/admin/meeting/" id "/slot/rm")
                (clj->js {:method "POST"
                          :headers {"Content-Type" "application/json"}
                          :body (.stringify js/JSON (clj->js slot))}))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (prn "removed" status))))))

(defn admin-login [password]
  (-> (js/fetch "/api/admin/login" (clj->js {:method "POST" :body password}))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   ; TODO show seconds left
                   200 (swap! state-atom dissoc :meeting-ids)

                   403 (swap! state-atom assoc :meeting-ids :forbidden)

                   (swap! state-atom assoc :meeting-ids :error)))))))

(defn admin-logout []
  (-> (js/fetch "/api/admin/logout")
      (.then (js/setTimeout
               (fn[]
                 (swap! state-atom dissoc :meeting-ids))
               500))))

(defn admin-meeting-sse [id]
  (let [sse (js/EventSource. (str "/api/admin/meeting/" id "/sse"))]
    (swap! state-atom assoc-in [:admin :sse id] sse)
    (set! (.-onmessage sse) (fn[event] 
                              (let [data (as-> event $
                                           (.-data $)
                                           (.parse js/JSON $)
                                           (js->clj $ {:keywordize-keys true}))]
                                (swap! state-atom assoc-in [:admin :meeting id] data))))))

(defn admin-stop-sse [id]
  (.close (get-in @state-atom [:admin :sse id]))
  (swap! state-atom update-in [:admin :sse] dissoc id))

(comment
  (admin-login "123")
  (admin-meeting-sse "alive")
  (admin-stop-sse "alive")
  (admin-logout)
  (fetch-meeting-list)
  @state-atom
  )
