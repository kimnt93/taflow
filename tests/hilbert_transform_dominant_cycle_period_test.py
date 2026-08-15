import numpy as np
import talib

from taflow import HilbertTransformDominantCyclePeriod


def _assert_lifecycle(values: np.ndarray) -> None:
    expected = talib.HT_DCPERIOD(values)
    full = HilbertTransformDominantCyclePeriod().extend(values)
    np.testing.assert_array_equal(full.compute(), expected)

    chunked = HilbertTransformDominantCyclePeriod()
    for chunk in np.array_split(values, 7):
        assert chunked.extend(chunk) is chunked
    np.testing.assert_array_equal(chunked.compute(), expected)
    assert chunked.value == full.value

    assert chunked.reset() is chunked
    assert chunked.value is None
    for value in values:
        assert chunked.append(float(value)) is chunked
    np.testing.assert_array_equal(chunked.compute(), expected)


def test_hilbert_transform_dominant_cycle_period_matches_talib_and_lifecycle():
    rng = np.random.default_rng(7391)
    _assert_lifecycle(100.0 + np.cumsum(rng.normal(0.0, 0.4, 257)))
    _assert_lifecycle(np.full(96, 12.5))
    _assert_lifecycle(np.linspace(100.0, 110.0, 129))
