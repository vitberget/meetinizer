(ns meetinizer.meeting.render-meeting
  (:require [meetinizer.the-state :refer [state-atom]])
  )

(defn date-from [timeline]
  (let [js-date (->> timeline 
                     (.parse js/Date)
                     (js/Date.))
        year (.getFullYear js-date)
        month (inc (.getMonth js-date))
        day (.getDate js-date)]
    (str year "-" (when (< month 10) "0") month "-" (when (< day 10) "0") day)))

(defn time-from [timeline]
  (let [js-date (->> timeline 
                     (.parse js/Date)
                     (js/Date.))
        hour (.getHours js-date)
        minute (.getMinutes js-date)]
    (str (when (< hour 10) "0") hour ":" (when (< minute 10) "0") minute)))

(defn votes-contains? [votes user slot]
  (->> votes
       (filter (fn[vote] (and
                           (= (:user_email vote) (:email user)) 
                           (= (:slot vote) slot))))
       (first)))

(defn render-actually [state 
                       {meeting-name :name 
                        slots :slots 
                        users :users 
                        votes :votes 
                        :as meeting}
                       my-user]
  (prn meeting)
  [:main.meet.meeting {:replicant/on-mount [[:meeting/monitor-meeting :start meeting-name]]}
   [:h1 "Meeting \"" meeting-name "\""]
   (when-let [comment (:comment meeting)]
     [:div.comment comment])
   [:table
    [:tr.header
     [:th ""]
     (->> slots
          (map (fn[slot] 
                 (let [start (:start slot)
                       end (:end slot)]
                   [:th 
                    [:div.dateheader
                     [:div.start (date-from start) " " (time-from start)]
                     [:div.end (date-from end) " " (time-from end)]]]))))]
    (->> users 
         (filter (fn[user] (not= user my-user)))
         (map (fn[user]
                [:tr
                 [:td.name (:name user)]
                 (->> slots
                      (map (fn[slot]
                             [:td.vote (if (votes-contains? votes user slot) "✓" "✗")])))])))

    [:tr.my-user
     [:td.name ">>" (:name my-user)]
     (->> slots
          (map (fn[slot]
                 (let [is-active (votes-contains? votes my-user slot)]
                   [:td.vote {:on {:click [[:meeting/set-vote slot (not is-active)]]}}
                    (if is-active "✓" "✗")]))))]]])


(comment
  @state-atom

  )
