import numpy as np
import wickra

from taflow import RollingConditionalValueAtRisk


def test_rolling_conditional_value_at_risk_matches_wickra_and_lifecycle() -> None:
    values = np.array([1.0, -1.0, 2.0, -2.0, 3.0, -3.0])
    expected = wickra.ConditionalValueAtRisk(3, 0.95).batch(values)
    batch = RollingConditionalValueAtRisk(timeperiod=3, confidence=0.95).extend(values)

    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(values)
    assert batch.value == expected[-1]

    streamed = RollingConditionalValueAtRisk(timeperiod=3, confidence=0.95)
    for value in values:
        assert streamed.append(value) is streamed
    np.testing.assert_array_equal(streamed.compute(), batch.compute())

    assert streamed.reset() is streamed
    assert len(streamed) == 0
    streamed.extend(values[:2]).extend(values[2:])
    np.testing.assert_array_equal(streamed.compute(), batch.compute())


def test_conditional_value_at_risk_uses_ranked_tail_count() -> None:
    values = np.linspace(-0.2, 0.2, 40)
    actual = RollingConditionalValueAtRisk(timeperiod=31).extend(values).compute()
    expected = wickra.ConditionalValueAtRisk(31).batch(values)
    np.testing.assert_allclose(actual, expected, equal_nan=True)
