(ns meetinizer.utils.cookie 
  (:require
   [clojure.string :as str]))

(defn get-cookie [cookie-name]
  (when-let [cookie (->> (str/split (.-cookie js/document) ";")
                       (map str/trim)
                       (filter (fn[cookie] (str/starts-with? cookie (str cookie-name "="))))
                       (first))]
    (-> (str/split cookie "=")
        (second)
        (js/decodeURIComponent))))

(comment
  (get-cookie "email")
  (subs "helo" 2)
  )
