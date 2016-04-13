module Main where

import Tyckiting.Position
import Test.Tasty
import Test.Tasty.QuickCheck
import Test.Tasty.HUnit

-- Tests are using `tasty` library
-- http://documentup.com/feuerbach/tasty
main :: IO ()
main = defaultMain tests

tests :: TestTree
tests = testGroup "Tests" [qcProps, unitTests]

-- These are property bases tests
qcProps :: TestTree
qcProps = testGroup "Properties (checked by QuickCheck)"
  [ testProperty "neighbours distance" $ 
      \d -> all ((<= getSmall d) . distance origo) (neighbours (getSmall d) origo)
  , testProperty "distance (clamp d pos) origo <= d" $
      \(Small d) x y -> distance (clamp d $ Position x y) origo <= d
  -- You can state single examples too
  , testProperty "there are 8 neighbours at distance 1" $ once $
      length (neighbours 1 origo) === 6
  , testProperty "there are 18 neighbours at distance 2" $ once $
      length (neighbours 2 origo) === 18
  ]

-- These are unit tests, ie just single assertions; could use IO
unitTests :: TestTree
unitTests = testGroup "Unit tests"
  [ testCase "there are 8 neighbours at distance 1" $
      length (neighbours 1 origo) @?= 6
  ]
