(ns meetinizer.eventhandler 
  (:require
   [clojure.walk :as walk]
   [meetinizer.admin.fetch :as af :refer [admin-meeting-sse admin-stop-sse]]
   [meetinizer.meeting.fetch :as mf :refer [add-vote meeting-sse rm-vote
                                            stop-sse]]
   [meetinizer.the-state :refer [get-path-parts path-part->meeting-id state-atom]]))

(defn- enrich-action-from-state [state action]
  (walk/postwalk
    (fn [x]
      (cond
        (and (vector? x)
             (= :db/get (first x))) (get state (second x))
        :else x))
    action))

(defn- enrich-action-from-event [{:replicant/keys [js-event node]} actions]
  (walk/postwalk
    (fn [x]
      (cond
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

(defn do-admin-login [password]
  (swap! state-atom assoc :meeting-ids :requesting-login)
  (af/admin-login password))

(defn- do-the-register-name [username]
  (let [meeting-id (path-part->meeting-id (get-path-parts))]
  (mf/register-name meeting-id username)))

(defn- do-admin-monitor-meeting [action id]
  (condp = action
    :start (admin-meeting-sse id)
    :stop (admin-stop-sse id)
    (prn "do-monitor-meeting no action for" action)))

(defn- do-monitor-meeting [action id]
  (prn "do-monitor-meeting" action id)
  (condp = action
    :start (when-not (get-in @state-atom [:sse id])
             (meeting-sse id))
    :stop (stop-sse id)
    (prn "do-monitor-meeting no action for" action)))

(defn do-set-vote [vote active-or-not]
  (let [meeting-id (path-part->meeting-id (get-path-parts))]
  (if active-or-not
    (add-vote meeting-id vote)
    (rm-vote meeting-id vote))))

(defn- do-admin-create-meeting [meeting-name]
  (af/create-meeting meeting-name))

(defn- do-admin-update-comment [meeting-name comment]
  (af/update-comment meeting-name comment))

(defn event-handler [{:replicant/keys [^js js-event] :as replicant-data} actions]
  (println actions)
  (doseq [action actions]
    (prn "Triggered action" action)
    (let [enriched-action (->> action
                               (enrich-action-from-event replicant-data)
                               (enrich-action-from-state @state-atom))
          [action-name & args] enriched-action]
      (prn "Enriched action" enriched-action)
      (prn "args" args)
      (condp = action-name
        :db/assoc (apply swap! state-atom assoc args)
        :db/dissoc (apply swap! state-atom dissoc args)

        :meeting/login (apply do-the-login args)
        :meeting/register-name (apply do-the-register-name args)
        :meeting/monitor-meeting (apply do-monitor-meeting args)
        :meeting/set-vote (apply do-set-vote args)

        :admin/login (apply do-admin-login args)
        :admin/logout (af/admin-logout)
        :admin/add-slot (apply af/add-slot args)
        :admin/rm-slot (apply af/rm-slot args)
        :admin/monitor-meeting (apply do-admin-monitor-meeting args)
        :admin/create-meeting (apply do-admin-create-meeting args)
        :admin/update-comment (apply do-admin-update-comment args)

        )))
  ; (main-thing el @state-atom)
  )

(comment
  @state-atom
  )
