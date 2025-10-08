(ns meetinizer.main
   (:require [replicant.dom :as r]))

(enable-console-print!)

(r/render js/document.body
  [:div.media
   [:main.grow
    [:h1 "Testing"]
    [:p "Playing with Replicant"] ]])
