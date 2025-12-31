(ns meetinizer.admin.render
  (:require
    [clojure.string :as s]
    [meetinizer.admin.fetch :refer [fetch-meeting fetch-meeting-list]]
    [meetinizer.meeting.render-meeting :refer [date-from time-from]]
    [meetinizer.the-state :refer [state-atom]]))

(defn render-loading [_]
  [:main.admin.loading
   [:h1 "Loading..."]])

(defn render-error [_]
  [:main.admin.loading
   [:h1 "Error"]])

(defn render-requesting [_]
  [:main.admin.loading
   [:h1 "Requesting login"]])

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
   [:input {:type "button" :value "Log out"
            :on {:click [[:admin/logout]]}}]
   (if (empty? meeting-ids)
     "No meetings yet"   
     [:ul (->> meeting-ids
               (map (fn[m] [:li [:input {:type "button"
                                         :value m
                                         :on {:click [[:db/assoc :admin/selected-meeting m]]}}]])))])])

(defn render-slots [{slots :slots id :name}]
  [:section.slots
   [:h2 "Slots"]
   [:div.slots
    (->> slots
         (map (fn[slot] [:div.slot 
                         [:div.from 
                          [:div.date (date-from (:start slot))]
                          [:div.date (time-from (:start slot))]]
                         [:div.to 
                          [:div.date (date-from (:end slot))]
                          [:div.date (time-from (:end slot))]]
                         [:div.action 
                          [:input {:type "button" 
                                   :value "Remove"
                                   :on {:click [[:admin/rm-slot id slot]]}}]
                          ] ])))
    [:div.slot.add
     [:div.from "Start"
      [:input {:type "datetime-local"
               :replicant/on-mount [[:db/assoc :admin/admin-slot-start-element :dom/node]]
               :on {:input [[:db/assoc :admin/login-slot-start :event/target.value]]} }]]
     [:div.to "End"
      [:input {:type "datetime-local"
               :replicant/on-mount [[:db/assoc :admin/admin-slot-end-element :dom/node]]
               :on {:input [[:db/assoc :admin/login-slot-end :event/target.value]]} }]] 
     [:div.action 
      [:input {:type "button" 
               :value "Add"
               :on {:click [[:admin/add-slot id [:db/get :admin/login-slot-start] [:db/get :admin/login-slot-end]]
                            [:db/dissoc :admin/login-slot-start]
                            [:db/dissoc :admin/login-slot-end]]}}]]]]])

(defn- render-users [{users :users votes :votes}]
  [:section.users
   [:h2 "Users"]
   [:table.users
    [:tr
     [:th "User"]
     [:th "Email"]
     [:th "Vote count"] ]
    (->> users
         (map (fn[{username :name email :email}] 
                [:tr 
                 [:td username]
                 [:td email]
                 [:td (->> votes
                           (filter (fn [{e :user_email}] (= email e)))
                           (count))]])))]
   [:h3 "All email"]
   [:div.emails
    [:div (->> users
               (map :email)
               (s/join ", "))]
    [:div (->> users
               (map (fn [{username :name email :email}] (str username " <" email ">" )))
               (s/join ", "))]]])

(defn render-meeting [{meeting-name :name :as meeting}]
  [:main.admin.meeting {:replicant/on-mount [[:admin/monitor-meeting :start meeting-name]]
                        :replicant/on-unmount [[:admin/monitor-meeting :stop meeting-name]] }
   [:h1 "Meeting: " meeting-name]
   [:input {:type "button"
            :value "Back to list"
            :on {:click [[:db/dissoc :admin/selected-meeting]]}}]
   (render-slots meeting)
   (render-users meeting)])

(defn render-admin [state]
  (let [meetings (:meeting-ids state)
        active-meeting (:admin/selected-meeting state)
        meeting (get-in state [:admin :meeting active-meeting])]
    (prn state)
    (cond 
      (nil? meetings)
      (do
        (fetch-meeting-list)
        (swap! state-atom assoc :meeting-ids :loading)
        (render-loading state)) 

      (= :loading meetings)
      (render-loading state)

      (= :error meetings)
      (render-error state)

      (= :forbidden meetings)
      (render-login state)

      (= :requesting-login meetings)
      (render-requesting state)

      (= :requested meetings)
      (render-requesting state)

      (and active-meeting (nil? meeting))
      (do
        (fetch-meeting active-meeting)
        (swap! state-atom assoc-in [:admin :meeting active-meeting] :loading)
        (render-loading state))

      (and active-meeting (= :loading meeting))
      (render-loading state)

      active-meeting
      (render-meeting meeting)

      :else
      (render-list state))))

(comment
  @state-atom
  (swap! state-atom dissoc :admin/selected-meeting)
  )
