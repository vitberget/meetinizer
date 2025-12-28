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
    (condp = (first path-parts)
      "meet" (r/render el (mr/render-meeting state))
      "admin" (r/render el (ar/render-admin state))

      (r/render el (status-404)) )))
