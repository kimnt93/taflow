import numpy as np
import wickra

from taflow import RollingProfitFactor


def test_rolling_profit_factor_matches_wickra_and_lifecycle() -> None:
    values = np.array([1.0, -1.0, 2.0, -2.0, 3.0, -3.0])
    expected = wickra.ProfitFactor(3).batch(values)
    batch = RollingProfitFactor(timeperiod=3).extend(values)

    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(values)
    assert batch.value == expected[-1]

    streamed = RollingProfitFactor(timeperiod=3)
    for value in values:
        assert streamed.append(value) is streamed
    np.testing.assert_array_equal(streamed.compute(), batch.compute())

    assert streamed.reset() is streamed
    assert len(streamed) == 0
    streamed.extend(values[:2]).extend(values[2:])
    np.testing.assert_array_equal(streamed.compute(), batch.compute())


def test_rolling_profit_factor_preserves_infinite_no_loss_result() -> None:
    values = np.array([0.01, 0.02, 0.03])
    actual = RollingProfitFactor(timeperiod=3).extend(values).compute()
    expected = wickra.ProfitFactor(3).batch(values)
    np.testing.assert_array_equal(actual, expected)
