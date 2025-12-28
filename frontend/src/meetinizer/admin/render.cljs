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
                        :replicant/on-mount [[:db/assoc :admin/login-form-element :dom/node]]
                        :on {:input [[:db/assoc :admin/login-form :event/target.value]]}}]
   [:input {:type "button" 
            :value "Login as admin"
            :on {:click [[:admin/login [:db/get :admin/login-form]]]}}]])

(defn render-list [{meeting-ids :meeting-ids}]
  [:main.admin.list
   [:h1 "Meetings"]
   (if (empty? meeting-ids)
     "No meetings yet"   
     [:ul (->> meeting-ids
               (map (fn[m] [:li [:input {:type "button"
                                         :value m
                                         :on {:click [[:db/assoc :admin/selected-meeting m]]}}]])))])])

(defn render-meeting [{active :admin/selected-meeting :as state}]
  [:main.admin.meeting
   [:h1 "You have chosen " active]
   ]
  )

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

      (:admin/selected-meeting state)
      (render-meeting state)
       
      :else
      (render-list state)

      ) 
    )
  )
(comment
  @state-atom
  (swap! state-atom dissoc :admin/selected-meeting)
  )
