(ns meetinizer.render 
  (:require
    [meetinizer.meeting.render :as mr]
    [meetinizer.admin.render :as ar]
    [meetinizer.the-state :refer [get-path-parts]]
    [replicant.dom :as r]))

(defn status-404 [] 
  [:main.no-route
   [:div [:h1 "404"]]
   "In the future, there will be some helpful text here."])

(defn main-thing [el state]
  (let [path js/window.location.pathname
        path-parts (get-path-parts)
        state (-> state 
                  (assoc :path path)
                  (assoc :path-parts path-parts))]
    (prn "path parts" path-parts)
    (prn "path parts second" (not (second path-parts)))
    (condp = (first path-parts)
      "meet" (if (second path-parts)
               (do
                 (mr/set-title state) 
                 (r/render el (mr/render-meeting state)))
               (r/render el (status-404)))

      "admin" (r/render el (ar/render-admin state))

      (r/render el (status-404)) )))
