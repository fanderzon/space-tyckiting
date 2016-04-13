module Console (
    module System.Console.ANSI,
    consoleLogBS,
    consoleLogString,
    ) where

import System.Console.ANSI
import qualified Data.ByteString.Lazy as B

-- | Log fancy message to the console.
consoleLogGen :: String -> Color -> String -> Maybe Color -> IO () -> IO ()
consoleLogGen pfx symColor sym bsColor action = do
    putStr pfx
    putStr " "
    setSGR [SetColor Foreground Vivid symColor]
    putStr sym
    setSGR []
    putStr " "
    case bsColor of
        Just bsColor' -> setSGR [SetColor Foreground Vivid bsColor']
        Nothing       -> return ()
    action
    setSGR []
    putStr "\n"

-- | Log fancy 'ByteString'.
consoleLogBS :: String -> Color -> String -> Maybe Color -> B.ByteString -> IO ()
consoleLogBS pfx symColor sym bsColor bs = consoleLogGen pfx symColor sym bsColor (B.putStr bs)

-- | Log fancy 'String'.
consoleLogString :: String -> Color -> String -> Maybe Color -> String -> IO ()
consoleLogString pfx symColor sym bsColor str = consoleLogGen pfx symColor sym bsColor (putStr str)
