;; For the clojure.test API please refer to:
;; https://clojure.github.io/clojure/clojure.test-api.html

(ns tyckiting.actions-test
  (:require [clojure.test :refer :all]
            [tyckiting.actions :refer :all]))

(def test-bots
  [{:bot-id 3
    :pos {:x 2 :y 4}}])

(deftest move-action
  (let [bot (first test-bots)
        dx 1 dy 2
        action (move bot dx dy)]

    (testing "sets the action type correctly"
      (is (= "move" (:type action))))

    (testing "moves the bot relative to it's current position"
      (is (= (+ (get-in bot [:pos :x]) dx) (get-in action [:pos :x])))
      (is (= (+ (get-in bot [:pos :y]) dy) (get-in action [:pos :y]))))))

(deftest cannon-action
  (let [bot (first test-bots)
        x 12 y 10
        action (cannon bot x y)]

    (testing "sets the action type correctly"
      (is (= "cannon" (:type action))))

    (testing "shoots at absolute position"
      (is (= x (get-in action [:pos :x])))
      (is (= y (get-in action [:pos :y]))))))
