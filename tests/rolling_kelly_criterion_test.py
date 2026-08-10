import numpy as np
import wickra

from taflow import RollingKellyCriterion


def test_rolling_kelly_criterion_matches_wickra_and_lifecycle() -> None:
    values = np.array([1.0, -1.0, 2.0, -2.0, 3.0, -3.0])
    expected = wickra.KellyCriterion(3).batch(values)
    batch = RollingKellyCriterion(values, timeperiod=3)

    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(values)
    assert batch.value == expected[-1]

    streamed = RollingKellyCriterion(np.array([], dtype=float), timeperiod=3)
    for value in values:
        assert streamed.append(value) is streamed
    np.testing.assert_array_equal(streamed.compute(), batch.compute())

    assert streamed.reset() is streamed
    assert len(streamed) == 0
    streamed.extend(values[:2]).extend(values[2:])
    np.testing.assert_array_equal(streamed.compute(), batch.compute())


def test_rolling_kelly_criterion_handles_no_loss_windows() -> None:
    values = np.array([0.0, 0.01, 0.02, 0.03])
    actual = RollingKellyCriterion(values, timeperiod=2).compute()
    expected = wickra.KellyCriterion(2).batch(values)
    np.testing.assert_array_equal(actual, expected)
