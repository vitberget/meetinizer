(ns meetinizer.render 
  (:require
   [clojure.string :as s]
   [meetinizer.meeting.render :as mr]
   [replicant.dom :as r]))

(defn status-404 [] [:div [:h1 "404"]])

(defn main-thing [el state]
  (let [path js/window.location.pathname
        path-parts (->> (s/split path "/")
                        (drop 1))
        state (-> state 
                  (assoc :path path)
                  (assoc :path-parts path-parts))]
    (condp = (first path-parts)
      "meet" (r/render el (mr/render-meeting state))

      :else (r/render el (status-404)) )))

