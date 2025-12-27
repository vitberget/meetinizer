(ns meetinizer.meeting.render
  (:require [meetinizer.meeting.fetch :refer [fetch-meeting]]
            [meetinizer.meeting.render-meeting :refer [render-actually]]
            [meetinizer.the-state :refer [state-atom]]))

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

(defn render-error [_]
  [:main.meeting.error
   [:h1 "Error!"]])

(defn render-login [{path-parts :path-parts}]
  [:main.meeting.login
   [:h1 "Login to meeting"]
   [:div.info "Login to meeting with id: " (second path-parts)]
   [:div.form
    [:label "Email:"
     [:input#login-email {:type "email"
                          :replicant/on-mount [[:db/assoc :meeting/login-form-element :dom/node]]
                          :on {:input [[:db/assoc :meeting/login-form :event/target.value]]}
                          }]]
    [:input {:type "button" 
             :value "Send me login mail"
             :on {:click [[:meeting/login [:db/get :meeting/login-form]]]}}]]])

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

      (= :error meeting)
      (render-error state)

      :else
      (render-actually state meeting)
      )))

(comment
  @state-atom
  )
