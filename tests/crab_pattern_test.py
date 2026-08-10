import numpy as np
import pytest

from taflow import CrabPattern


def test_crab_pattern_lifecycle_and_alignment() -> None:
    length = 6
    open_ = np.full(length, 10.0)
    high = np.arange(12.0, 12.0 + length)
    low = np.arange(8.0, 8.0 - length, -1.0)
    close = np.arange(10.0, 10.0 + length)

    indicator = CrabPattern(open_, high, low, close)
    assert len(indicator) == length
    assert np.isnan(indicator.compute()[:5]).all()
    assert indicator.value is not None

    replay = indicator.compute().copy()
    indicator.reset().extend(open_, high, low, close)
    np.testing.assert_array_equal(indicator.compute(), replay)

    with pytest.raises(ValueError):
        CrabPattern([1.0], [2.0, 3.0], [0.0], [1.0])
