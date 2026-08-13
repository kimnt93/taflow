import numpy as np
from taflow import Vortex


def test_vortex_lifecycle() -> None:
    close = 100.0 + np.sin(np.arange(64.0) / 5.0)
    state = Vortex(window=5).extend(close + 1.0, close - 1.0, close)
    first = state.compute()
    state.reset().extend(close + 1.0, close - 1.0, close)
    for got, expected in zip(state.compute(), first): np.testing.assert_array_equal(got, expected)
    assert len(state) == len(close)

