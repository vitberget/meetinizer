(ns meetinizer.meeting.render
  (:require [meetinizer.meeting.fetch :refer [fetch-meeting]]
            [meetinizer.the-state :refer [state-atom]]))

(defn render-actually [state]
  [:main.meeting 
   [:h1 "Hello everywhere"]
   "Your are at " (:path state)  
   ])

(defn render-requesting [_]
  [:main.meeting.requesting 
   [:h1 "Requesting login email"]])

(defn render-requested [state]
  (let [meeting-id (-> state
                       (:path-parts)
                       (second))
        seconds (get-in state [:meeting meeting-id :requested])]
    (prn meeting-id)
    [:main.meeting.reqeusted
     [:h1 "Login mail sent"]
     [:div "Check your email inbox!"]
     [:div "Valid for " seconds " seconds."]]))

(defn render-loading [_]
  [:main.meeting.loading
   [:h1 "Loading..."]])

(defn render-login [{path-parts :path-parts}]
  [:main.meeting.login
   [:h1 "Login to meeting"]
   [:div.info "Login to meeting with id: " (second path-parts)]
   [:div.form
    [:label "Email:"
     [:input#login-email {:type "email"
                          :replicant/on-mount [[:db/assoc :login/form-element :dom/node]]
                          :on {:input [[:db/assoc :login/form :event/target.value]]}
                          }]]
    [:input {:type "button" 
             :value "Send me login mail"
             :on {:click [[:auth/login [:db/get :login/form] ]]}
             }]]])

(defn render-meeting [state]
  (let [meeting-id (second (:path-parts state))
        meeting (get-in state [:meeting meeting-id])]
    (cond
      (nil? meeting)
      (do (fetch-meeting meeting-id)
          (swap! state-atom assoc-in [:meeting meeting-id] :loading)
          (render-loading state))

      (= :loading meeting)
      (render-loading state)

      (= :requesting meeting)
      (render-requesting state)

      (:requested meeting)
      (render-requested state)

      (= :forbidden meeting)
      (render-login state)


      :else
      (render-actually state)
      )))

(comment
  @state-atom
  )
