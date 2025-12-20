(ns meetinizer.main
  (:require [replicant.dom :as r]
            [meetinizer.the-state :refer [state-atom]]
  ))

(defn test-replicant [state]
  [:div "Hello there"])

(defn main-thing [el state]
  (r/render el (test-replicant state))
  )


(defonce el (js/document.getElementById "app"))

(defn ^:dev/after-load main []
  (add-watch state-atom ::render 
             (fn [_ _ _ state]
               (main-thing el state)))
  (main-thing el @state-atom)
  )

