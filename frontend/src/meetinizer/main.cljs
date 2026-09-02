(ns meetinizer.main
  (:require
    [meetinizer.eventhandler :refer [event-handler]]
    [meetinizer.render :refer [main-thing]]
    [meetinizer.the-state :refer [state-atom]]
    [replicant.dom :as r]))

(defonce el (js/document.getElementById "app"))

(comment @state-atom)

(defn ^{:dev/after-load true :export true} main! []
  (add-watch state-atom ::render
             (fn [_ _ _ state]
               (main-thing el state)))
  (main-thing el @state-atom))

(defn ^:export init! []
  (r/set-dispatch! event-handler)
  (main!))
