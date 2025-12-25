(ns meetinizer.meeting.render
  (:require [meetinizer.meeting.fetch :refer [fetch-meeting]]
            [meetinizer.the-state :refer [state-atom]]))

(defn render-actually [state]
  [:div 
   [:h1 "Hello everywhere"]
   "Your are at " (:path state)  
   ])
(defn render-loading [_]
  [:div [:h1 "Loading..."]])

(defn render-login [_]
  [:div [:h1 "Login"]
   [:label "Email:"
    [:input#login-email {:type "email"
                         :replicant/on-mount [[:db/assoc :login/form-element :dom/node]]
                         :on {:input [[:db/assoc :login/form :event/target.value]]}
                         }]]
   [:input {:type "button" 
            :value "Send me login"
            :on {:click [[:auth/login]]}
            }]])

            (defn render-meeting [state]
              (let [meeting-id (second (:path-parts state))
                    meeting (get-in state [:meeting meeting-id])]
                ; (println "rm")
                ; (println state)
                (cond
                  (nil? meeting)
                  (do (fetch-meeting meeting-id)
                      (swap! state-atom assoc-in [:meeting meeting-id] :loading)
                      (render-loading state))

                  (= :loading meeting)
                  (render-loading state)

                  (= :forbidden meeting)
                  (render-login state)


                  :else
                  (render-actually state)
                  )))

            (comment
              @state-atom
              )
