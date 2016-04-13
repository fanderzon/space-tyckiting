module Options.Applicative.Extras (
    module Options.Applicative,
    lookupReader,
    ) where

import Data.List                 (intercalate)
import Options.Applicative.Types (readerAsk)
import Options.Applicative

lookupReader :: [(String, a)] -> ReadM a
lookupReader opts = readerAsk >>= lookup'
  where
    lookup' s = maybe (readerError err) return $ lookup s opts
      where
        err = concat [ "Unknown option: "
                     , s
                     , ", available: "
                     , intercalate ", " (map fst opts)
                     ]
