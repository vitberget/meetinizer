(ns meetinizer.the-state 
  (:require
   [clojure.string :as s]))

(defn get-path-parts []
  (let [path js/window.location.pathname
        path-parts (->> (s/split path "/")
                        (drop 1))]
    path-parts))

(defonce state-atom (atom {}))

