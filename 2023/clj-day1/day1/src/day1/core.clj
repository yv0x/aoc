(ns day1.core
  (:require [clojure.java.io :as io]))

(defn extract-last-digits [line]
  (let [numbers (re-seq #"\d+" line)]
    (when (seq numbers)
      (map #(-> % last (str) (first)) numbers))))

(defn process-file [file-path]
  (try
    (with-open [reader (io/reader file-path)]
      (let [lines (line-seq reader)
            last-digits (remove empty? (map extract-last-digits lines))]
        (println "Last Digits of Numbers in Each Line:")
        (doseq [digits last-digits]
          (println digits))))
    (catch Exception e
      (println (str "Error reading the file: " (.getMessage e))))))

(defn -main [input-file]
  (let [file-path (str input-file)]
    (if (.exists (java.io.File. file-path))
      (process-file file-path)
      (println "File not found."))))

(defn run []
  (if (empty? *command-line-args*)
    (println "Usage: Please provide the input file path as an argument.")
    (-main (first *command-line-args*))))

