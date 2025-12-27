(ns meetinizer.admin.fetch
  (:require
   [meetinizer.the-state :refer [state-atom]]))

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
; const response = await fetch("https://example.org/post", {
;   method: "POST",
;   body: JSON.stringify({ username: "example" }),
;   // …
; });
(defn admin-login [password]
  (-> (js/fetch "/api/admin/login" (clj->js {:method "POST" :body password}))
      (.then (fn [the-result]
               (let [status (.-status the-result)]
                 (condp = status
                   ; TODO show seconds left
                   200 (-> (.text the-result) 
                           (.then (fn[text]
                                    (swap! state-atom assoc :meeting-ids :requested ))))

                   403 (swap! state-atom assoc :meeting-ids :forbidden)

                   (swap! state-atom assoc :meeting-ids :error)))))))

(defn admin-logout []
  (-> (js/fetch "/api/admin/logout")
      (.then (fn[fetch-result]
               (prn "Logged out?")
               ))))

(comment
  (admin-login "123")
  (admin-logout)
  )
