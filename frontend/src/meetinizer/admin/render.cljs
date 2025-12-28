(ns meetinizer.admin.render
  (:require
   [meetinizer.admin.fetch :refer [fetch-meeting fetch-meeting-list]]
   [meetinizer.the-state :refer [state-atom]]))

(defn render-loading [_]
  [:main.admin.loading
   [:h1 "Loading..."]])

(defn render-error [_]
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

(defn render-meeting [meeting]
  (prn "Meeting")
  (prn meeting)
  [:main.admin.meeting
   [:h1 "You have chosen: " (:name meeting)]
   [:section.slots
    [:h2 "Slots"]
    (->> meeting
         (:slots)
         (map (fn[slot] [:div.slot "hello"]))
         ) ] ])

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
