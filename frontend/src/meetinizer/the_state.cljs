(ns meetinizer.the-state 
  (:require
   [clojure.string :as s]))

(defn get-path-parts []
  (let [path js/window.location.pathname
        path-parts (->> (s/split path "/")
                        (drop 1))]
    path-parts))

(defn path-part->meeting-id [path-parts]
  (-> path-parts
      (second)
      (js/decodeURIComponent)))

(defonce state-atom (atom {}))

