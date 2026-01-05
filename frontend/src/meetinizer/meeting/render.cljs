(ns meetinizer.meeting.render
  (:require
    [meetinizer.meeting.fetch :refer [fetch-meeting]]
    [meetinizer.meeting.render-meeting :refer [render-actually]]
    [meetinizer.the-state :refer [path-part->meeting-id state-atom]]
    [meetinizer.utils.cookie :refer [get-cookie]]))

(defn render-requesting [_]
  [:main.meet.requesting 
   [:h1 "Requesting login email"]])

(defn render-requested [state]
  (let [meeting-id (-> state
                       (:path-parts)
                       (path-part->meeting-id))
        seconds (get-in state [:meeting meeting-id :requested])]
    (prn meeting-id)
    [:main.meet.reqeusted
     [:h1 "Login mail sent"]
     [:div "Check your email inbox!"]
     [:div "Valid for " seconds " seconds."]]))

(defn render-loading [_]
  [:main.meet.loading
   [:h1 "Loading..."]])

(defn render-error [_]
  [:main.meet.error
   [:h1 "Error!"]])

(defn render-login [{path-parts :path-parts}]
  (let [meeting-id (path-part->meeting-id path-parts)]
    [:main.meet.login
     [:h1 "Login to meeting"]
     [:div.gdpr 
      [:p "When you request to login, an email will be sent to your email with a link which you "
       "can log in with."]
      [:p "After you login, you will have to select a name/alias. Your email and chosen name/alias will be "
       "stored in a database. Your email will also be stored in a login cookie. Both email and chosen "
       "name/alias will be visible to the administrator."]
      [:p "Other users will be able to see your chosen name/alias but not your email."]]
     [:div.info "Login to " meeting-id]
     [:div.form
      [:label "Email:"
       [:input#login-email {:type "email"
                            :replicant/on-mount [[:db/assoc :meeting/login-form-element :dom/node]]
                            :on {:input [[:db/assoc :meeting/login-form :event/target.value]]} }]]
      [:input {:type "button" 
               :value "Send me login mail"
               :on {:click [[:meeting/login [:db/get :meeting/login-form]]]}}]]]))

(defn render-enter-name [state {meeting-name :name}]
  [:main.meet.enter-name {:replicant/on-mount [[:meeting/monitor-meeting :start meeting-name]]}
   [:h1 "Welcome, who are you?"]
   [:input#login-email {:type "text"
                        :replicant/on-mount [[:db/assoc :meeting/name-form-element :dom/node]]
                        :on {:input [[:db/assoc :meeting/name-form :event/target.value]]}
                        }]
   [:input {:type "button" 
            :value "Register name"
            :on {:click [[:meeting/register-name [:db/get :meeting/name-form]]]}}]])


(defn render-meeting [state]
  (let [meeting-id (path-part->meeting-id (:path-parts state))
        meeting (get-in state [:meeting meeting-id])
        my-email (get-cookie "email")]
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
      (if-let [my-user (->> meeting
                            (:users)
                            (filter (fn [{email :email}] (= email my-email)))
                            (first))]
        (render-actually state meeting my-user)
        (render-enter-name state meeting)))))

(defn set-title [state]
  (let [meeting-id (path-part->meeting-id (:path-parts state))
        meeting (get-in state [:meeting meeting-id])
        my-email (get-cookie "email")]
    (prn state)
    (cond
      (nil? meeting)
      (set! js/document.title "Meetinizer | Loading")

      (= :loading meeting)
      (set! js/document.title "Meetinizer | Loading")

      (= :requesting meeting)
      (set! js/document.title "Meetinizer | Requesting email")

      (:requested meeting)
      (set! js/document.title "Meetinizer | Requested email")

      (= :forbidden meeting)
      (set! js/document.title "Meetinizer | Login")

      (= :error meeting)
      (set! js/document.title "Meetinizer | Error")

      :else
      (if-let [my-user (->> meeting
                            (:users)
                            (filter (fn [{email :email}] (= email my-email)))
                            (first))]
        (set! js/document.title "Meetinizer | Meeting")
        (set! js/document.title "Meetinizer | Enter name")))))

(comment
  @state-atom
  )
