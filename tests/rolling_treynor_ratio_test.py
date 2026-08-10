import numpy as np
import wickra

from taflow import RollingTreynorRatio


def test_rolling_treynor_ratio_matches_wickra_and_lifecycle() -> None:
    values = np.array([1.0, -1.0, 2.0, -2.0, 3.0, -3.0])
    benchmark = np.array([0.5, -0.5, 1.0, -1.0, 1.5, -1.5])
    expected = wickra.TreynorRatio(3).batch(values, benchmark)
    batch = RollingTreynorRatio(values, benchmark, timeperiod=3)

    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(values)
    assert batch.value == expected[-1]

    empty = np.array([], dtype=float)
    streamed = RollingTreynorRatio(empty, empty, timeperiod=3)
    for pair in zip(values, benchmark, strict=True):
        assert streamed.append(*pair) is streamed
    np.testing.assert_array_equal(streamed.compute(), batch.compute())

    assert streamed.reset() is streamed
    streamed.extend(values[:2], benchmark[:2])
    streamed.extend(values[2:], benchmark[2:])
    np.testing.assert_array_equal(streamed.compute(), batch.compute())


def test_rolling_treynor_ratio_rejects_misaligned_input() -> None:
    with np.testing.assert_raises(ValueError):
        RollingTreynorRatio([1.0, 2.0], [1.0], timeperiod=3)
