(ns meetinizer.admin.render
  (:require [meetinizer.the-state :refer [state-atom]]
            [meetinizer.admin.fetch :refer [fetch-meeting-list]]))

(defn render-loading [_]
  [:main.admin.loading
   [:h1 "Loading..."]])

(defn render-login [_]
  [:main.admin.login
   [:h1 "Enter admin password"]
   [:input#login-email {:type "email"
                        :replicant/on-mount [[:db/assoc :admin-login/form-element :dom/node]]
                        :on {:input [[:db/assoc :admin/login-form :event/target.value]]}
                        }]
   [:input {:type "button" 
            :value "Login as admin"
            :on {:click [[:admin/login [:db/get :admin/login-form]]]}}]
   ])

(defn render-list [state]
  [:main.admin.list
   [:h1 "Meetings"]
   [:ul
    [:li "Meet 1"]
    [:li "Meet 2"]
    [:li "Meet 3"]]])

(defn render-admin [state]
  (let [meetings (:meeting-ids state)]
    (cond 
      (nil? meetings)
      (do
        (fetch-meeting-list)
        (swap! state-atom assoc :meeting-ids :loading)
        (render-loading state)) 

      (= :loading meetings)
      (render-loading state)

      (= :forbidden meetings)
      (render-login state)

      :else
      (render-list state)

      ) 
    )
  )
