import numpy as np

from taflow import RollingRecoveryFactor


def test_rolling_recovery_factor_lifecycle_is_invariant() -> None:
    """The fixed-window contract is a documented Wickra RecoveryFactor variant."""
    equity = np.array([1.0, 2.0, 1.0, 3.0, 2.0, 4.0])
    batch = RollingRecoveryFactor(equity, timeperiod=3)

    streamed = RollingRecoveryFactor(np.array([], dtype=float), timeperiod=3)
    for value in equity:
        assert streamed.append(value) is streamed
    np.testing.assert_array_equal(streamed.compute(), batch.compute())
    assert streamed.value == batch.value
    assert len(streamed) == len(equity)

    assert streamed.reset() is streamed
    assert len(streamed) == 0
    streamed.extend(equity[:2]).extend(equity[2:])
    np.testing.assert_array_equal(streamed.compute(), batch.compute())
