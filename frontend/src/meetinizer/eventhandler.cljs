(ns meetinizer.eventhandler 
  (:require [clojure.walk :as walk]
            [meetinizer.admin.fetch :as af]
            [meetinizer.meeting.fetch :as mf]
            [meetinizer.the-state :refer [get-path-parts path-part->meeting-id state-atom]]))

(defn- enrich-action-from-state [state action]
  (walk/postwalk
    (fn [x] (cond (and (vector? x)
                       (= :db/get (first x))) 
                  (get state (second x))
                  :else x))
    action))

(defn- enrich-action-from-event [{:replicant/keys [js-event node]} actions]
  (walk/postwalk
    (fn [x] (cond
              (keyword? x)
              (case x
                :event/target.value (-> js-event .-target .-value)
                :dom/node node
                x)
              :else x))
    actions))

(defn do-the-login [email]
  (let [meeting-id (path-part->meeting-id (get-path-parts))]
    (swap! state-atom assoc-in [:meeting meeting-id] :requesting)
    (mf/login meeting-id email)))

(defn- do-the-register-name [username]
  (let [meeting-id (path-part->meeting-id (get-path-parts))]
    (mf/register-name meeting-id username)))

(defn- do-admin-monitor-meeting [action id]
  (condp = action
    :start (af/admin-meeting-sse id)
    :stop (af/admin-stop-sse id)
    (prn "do-monitor-meeting no action for" action)))

(defn- do-monitor-meeting [action id]
  (condp = action
    :start (when-not (get-in @state-atom [:sse id]) (mf/meeting-sse id))
    :stop (mf/stop-sse id)
    (prn "do-monitor-meeting no action for" action)))

(defn do-set-vote [vote active-or-not]
  (let [meeting-id (path-part->meeting-id (get-path-parts))]
    (if active-or-not
      (mf/add-vote meeting-id vote)
      (mf/rm-vote meeting-id vote))))

(defn- admin-update-password [replicant-data]
  (let [password-text (-> replicant-data 
                          (:replicant/node)
                          (.-value))]
    (swap! state-atom assoc :admin/password-text password-text)))

(defn do-admin-login []
  (swap! state-atom assoc :meeting-ids :requesting-login)
  (let [password-text (:admin/password-text @state-atom)]
    (af/admin-login password-text)))

(defn admin-update-password-keydown [replicant-data]
  (let [code (-> replicant-data
                 (:replicant/dom-event)
                 (.-code))]
    (when (= code "Enter")
      (do-admin-login))))

(defn event-handler [{:replicant/keys [^js js-event] :as replicant-data} actions]
  (doseq [action actions]
    (let [enriched-action (->> action
                               (enrich-action-from-event replicant-data)
                               (enrich-action-from-state @state-atom))
          [action-name & args] enriched-action]
      (condp = action-name
        :db/assoc (apply swap! state-atom assoc args)
        :db/dissoc (apply swap! state-atom dissoc args)

        :meeting/login (apply do-the-login args)
        :meeting/logout (apply mf/logout args)
        :meeting/register-name (apply do-the-register-name args)
        :meeting/monitor-meeting (apply do-monitor-meeting args)
        :meeting/set-vote (apply do-set-vote args)

        :admin/update-password (admin-update-password replicant-data)
        :admin/update-password-keydown (admin-update-password-keydown replicant-data)
        :admin/assoc-password-element (swap! state-atom assoc :admin/password-element (first args))
        :admin/login (do-admin-login)
        :admin/logout (af/admin-logout)
        :admin/lock (apply af/lock args)
        :admin/select-slot (apply af/select-slot args)
        :admin/deselect-slot (apply af/deselect-slot args)

        :admin/add-slot (apply af/add-slot args)
        :admin/rm-slot (apply af/rm-slot args)
        :admin/monitor-meeting (apply do-admin-monitor-meeting args)
        :admin/create-meeting (apply af/create-meeting args)
        :admin/update-comment (apply af/update-comment args)

        )))
  ; (main-thing el @state-atom)
  )

(comment
  @state-atom
  )
