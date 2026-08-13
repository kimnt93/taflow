import numpy as np
import talib

from taflow import ChandeMomentumOscillator


def test_matches_talib_cmo() -> None:
    values = 100.0 + np.arange(128) * 0.2 + np.sin(np.arange(128) * 0.17)
    np.testing.assert_allclose(ChandeMomentumOscillator(14).extend(values).compute(), talib.CMO(values, 14), equal_nan=True)
