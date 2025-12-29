(ns meetinizer.admin.fetch
  (:require
   [meetinizer.the-state :refer [state-atom]]))

(defn fetch-meeting [id]
  (-> (js/fetch (str "/api/admin/meeting/" id))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   200 (-> (.json the-result)
                           (.then (fn [json]
                                    (let [data (js->clj json :keywordize-keys true)]
                                      (swap! state-atom assoc-in [:admin :meeting id] data)))))

                   403 (swap! state-atom assoc-in [:admin :meeting id] :forbidden)

                   (swap! state-atom assoc-in [:admin :meeting id] :error)))))))

(defn fetch-meeting-list []
  (-> (js/fetch "/api/admin/list")
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   200 (-> (.json the-result)
                           (.then (fn [json]
                                    (let [data (js->clj json :keywordize-keys true)]
                                      (swap! state-atom assoc-in [:meeting-ids] data)))))

                   403 (swap! state-atom assoc-in [:meeting-ids] :forbidden)

                   (swap! state-atom assoc-in [:meeting-ids] :error)))))))

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

(comment
  (admin-login "123")
  (admin-logout)
  (fetch-meeting-list)
  @state-atom
  )
