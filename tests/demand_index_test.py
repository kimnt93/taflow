import numpy as np
import wickra

from taflow import DemandIndex


def test_demand_index_matches_wickra_and_lifecycle() -> None:
    close = np.array([10.0, 11.0, 10.0, 12.0, 11.0, 13.0, 12.0])
    high = close + 1.0
    low = close - 1.0
    volume = np.array([100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0])
    expected = wickra.DemandIndex(3).batch(high, low, close, volume)

    batch = DemandIndex(high, low, close, volume, timeperiod=3)
    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(close)
    assert batch.value is not None
    assert np.isclose(batch.value, expected[-1])

    empty = np.array([], dtype=float)
    chunked = DemandIndex(empty, empty, empty, empty, timeperiod=3)
    chunked.extend(high[:2], low[:2], close[:2], volume[:2])
    chunked.extend(high[2:], low[2:], close[2:], volume[2:])
    np.testing.assert_array_equal(chunked.compute(), batch.compute())

    assert chunked.reset() is chunked
    for bar in zip(high, low, close, volume, strict=True):
        assert chunked.append(*bar) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())


def test_demand_index_rejects_misaligned_input() -> None:
    with np.testing.assert_raises(ValueError):
        DemandIndex([2.0], [0.0], [1.0], [10.0, 11.0])
