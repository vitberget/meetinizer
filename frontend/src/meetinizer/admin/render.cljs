(ns meetinizer.admin.render
  (:require
    [clojure.string :as s]
    [meetinizer.admin.fetch :refer [fetch-meeting fetch-meeting-list]]
    [meetinizer.meeting.render-meeting :as rm]
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
   [:input#login-email {:type "password"
                        :autofocus true
                        :replicant/on-mount [[:admin/assoc-password-element :dom/node]]
                        :on {:input [[:admin/update-password]]
                             :keydown [[:admin/update-password-keydown]] 
                             }}]
   [:input {:type "button" 
            :value "Login as admin"
            :on {:click [[:admin/login]]}}]])

(defn render-list [{meeting-ids :meeting-ids}]
  [:main.admin.list
   [:h1 "Meetings"]
   [:input {:id "logout"
            :type "button" 
            :value "Log out"
            :on {:click [[:admin/logout]]}}]
   (if (empty? meeting-ids)
     "No meetings yet"   
     [:ul (->> meeting-ids
               (map (fn[m] [:li 
                            [:span m]
                            [:input {:type "button"
                                     :value "View"
                                     :on {:click [[:db/assoc :admin/selected-meeting m]]}}]])))])
   [:div.new-meeting
    [:input {:type "text"
             :on {:input [[:db/assoc :admin/create-meeting-form :event/target.value]]}}]
    [:input {:type "button" 
             :value "Create new meeting"
             :on {:click [[:admin/create-meeting [:db/get :admin/create-meeting-form]]
                          [:db/assoc :admin/create-meeting-form ""]]}}]]])

(defn render-slots [{slots :slots id :name votes :votes chosen-slot :chosen_slot}]
  [:section.slots
   [:h2 "Slots"]
   [:div.slots
    (->> slots
         (rm/sort-slots)
         (map (fn[slot] [:div.slot 
                         [:div.from 
                          [:div.date (rm/date-from (:start slot))]
                          [:div.date (rm/time-from (:start slot))]]
                         [:div.to 
                          [:div.date (rm/date-from (:end slot))]
                          [:div.date (rm/time-from (:end slot))]]
                         [:div.count
                          (->> votes
                               (filter (fn[vote] (= (:slot vote) slot)))
                               (count))
                          " vote(s)"]
                         [:div.action 
                          (if (= chosen-slot slot)
                            [:input {:type "button" 
                                     :value "Deselect"
                                     :on {:click [[:admin/deselect-slot id slot]]}}]
                            [:input {:type "button" 
                                     :value "Select"
                                     :on {:click [[:admin/select-slot id slot]]}}])
                          [:input {:type "button" 
                                   :value "Remove"
                                   :on {:click [[:admin/rm-slot id slot]]}}]]])))
    [:div.slot.add
     [:div.from "Start"
      [:input {:type "datetime-local"
               :replicant/on-mount [[:db/assoc :admin/admin-slot-start-element :dom/node]]
               :on {:input [[:db/assoc :admin/login-slot-start :event/target.value]]}}]]
     [:div.to "End"
      [:input {:type "datetime-local"
               :replicant/on-mount [[:db/assoc :admin/admin-slot-end-element :dom/node]]
               :on {:input [[:db/assoc :admin/login-slot-end :event/target.value]]}}]] 
     [:div.action 
      [:input {:type "button" 
               :value "Add"
               :on {:click [[:admin/add-slot id [:db/get :admin/login-slot-start] [:db/get :admin/login-slot-end]]
                            [:db/dissoc :admin/login-slot-start]
                            [:db/dissoc :admin/login-slot-end]]}}]]]]])

(defn- render-users [{users :users votes :votes}]
  (let [users (sort-by :name users)]
    [:section.users
     [:h2 "Users"]
     [:table.users
      [:tr
       [:th "User"]
       [:th "Email"]
       [:th "Vote count"]]
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
      (let [emails (->> users
                        (map :email)
                        (s/join ", "))]
        [:div 
         [:div emails] 
         [:input {:type "button"
                  :value "Copy"
                  :on {:click [[:util/copy-to-clipboard emails]]}}]])
      (let [emails (->> users
                        (map (fn [{username :name email :email}] (str username " <" email ">" )))
                        (s/join ", "))]
        [:div 
         [:div emails]
         [:input {:type "button"
                  :value "Copy"
                  :on {:click [[:util/copy-to-clipboard emails]]}}]])]]))

(defn- render-comment [{comment-text :comment id :name}]
  [:section.comment
   [:h2 "Comment"]
   [:textarea {:cols "80" 
               :rows "20"
               :on {:input [[:db/assoc :admin/comment :event/target.value]]}}
    comment-text]  
   [:input {:type "button"
            :value "Update comment"
            :on {:click [[:admin/update-comment id [:db/get :admin/comment]]]}}]])

(defn- render-lock [{id :name locked :locked}]
  [:section.lock
   [:h2 "Lock"] 
   [:input {:type "button"
            :value "Lock"
            :on {:click [[:admin/lock id true]]}
            :disabled locked}]
   [:input {:type "button"
            :value "Unlock"
            :on {:click [[:admin/lock id false]]}
            :disabled (not locked)}]])

(defn render-meeting [{meeting-name :name :as meeting}]
  [:main.admin.meeting 
   [:h1 "Meeting: " meeting-name]
   [:div.hidden-lifetime {:replicant/on-mount [[:admin/monitor-meeting :start meeting-name]]
                          :replicant/on-unmount [[:admin/monitor-meeting :stop meeting-name]]}]
   [:input {:id "back-to-list"
            :type "button"
            :value "Back to list"
            :on {:click [[:db/dissoc :admin/selected-meeting]]}}]
   (render-comment meeting)
   (render-lock meeting)
   (render-slots meeting)
   (rm/render-vote-table meeting nil)
   (render-users meeting)])

(defn render-admin [state]
  (let [meetings (:meeting-ids state)
        active-meeting (:admin/selected-meeting state)
        meeting (get-in state [:admin :meeting active-meeting])]
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
