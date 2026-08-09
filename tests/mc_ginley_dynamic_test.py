import numpy as np
from taflow import McGinleyDynamic


def test_mcginley_dynamic_lifecycle() -> None:
    close = 100.0 + np.arange(64.0)
    state = McGinleyDynamic(close, length=10, c=0.6)
    first = state.compute()
    state.reset().extend(close)
    np.testing.assert_array_equal(state.compute(), first)

