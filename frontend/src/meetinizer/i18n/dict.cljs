(ns meetinizer.i18n.dict
  (:require [tongue.core :as tongue]))

(def dicts
  {:en {:hello "Hello"}
   :sv {:hello "Hej"}

   :tongue/fallback :en})

(def translate ;; [locale key & args] => string
  (tongue/build-translate dicts))

(comment
  (translate :sv :hello)
  )
