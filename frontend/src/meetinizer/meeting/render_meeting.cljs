(ns meetinizer.meeting.render-meeting
  (:require [meetinizer.the-state :refer [state-atom]]
            [clojure.string :as str]))

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

(defn render-slot-header [{start :start end :end}]
  (let [date-from-start (date-from start)
        date-from-end (date-from end)
        time-from-start (time-from start)
        time-from-end (time-from end)]
    [:th 
     (cond
       (and (= date-from-start date-from-end)
            (= time-from-start time-from-end))
       [:div.dateheader
        [:div.start date-from-start]]

       (= date-from-start date-from-end)
       [:div.dateheader
        [:div.start date-from-start]
        [:div.end time-from-start " - " time-from-end]]

       :else 
       [:div.dateheader
        [:div.start date-from-start " " time-from-start]
        [:div.end date-from-end " " time-from-end]])]))

(defn sort-slots [slots]
  (->> slots
       (sort (fn [a b] (let [c (compare (:start a) (:start b))]
                         (if (zero? c)
                           (compare (:end a) (:end b))
                           c))))))

(defn render-vote-table [{slots :slots 
                          users :users 
                          votes :votes}
                         my-user]
  (let [slots (sort-slots slots)]
    [:table.vote-table
     [:tr.header
      [:th.blank ]
      (->> slots (map render-slot-header))]
     (->> users 
          (filter (fn[user] (not= user my-user)))
          (sort (fn [a b] (compare (:name a) (:name b))))
          (map (fn[user]
                 [:tr.other-user
                  [:td.name (:name user)]
                  (->> slots
                       (map (fn[slot]
                              (let [is-active (votes-contains? votes user slot)]
                                [:td.vote [:div.vote
                                           {:class (if is-active ["vote" "active"] ["vote"])} 
                                           (if is-active "✓" "✗")]]))))]))))
     (when my-user
       [:tr.my-user
        [:td.name (:name my-user)]
        (->> slots
             (map (fn[slot]
                    (let [is-active (votes-contains? votes my-user slot)]
                      [:td.vote {:on {:click [[:meeting/set-vote slot (not is-active)]]}}
                       [:div.vote 
                        {:class (if is-active ["vote" "active"] ["vote"])} 
                        (if is-active "✓" "✗")]]))))])])

(defn render-actually [_ {meeting-name :name :as meeting} my-user]
  [:main.meet.meeting 
   [:h1 meeting-name]
   [:div.hidden-lifetime {:replicant/on-mount [[:meeting/monitor-meeting :start meeting-name]]}]

   (when-let [comment-text (:comment meeting)]
     [:div.comment (as-> comment-text $
                     (str/split $ "\n\n")
                     (map (fn [p] [:p p]) $))])
   (render-vote-table meeting my-user)
   [:div.logout 
    [:input {:type "button"
             :value "Log out"
             :on {:click [[:meeting/logout meeting-name]]}}]]])


(comment
  @state-atom
  "fd"

  )
