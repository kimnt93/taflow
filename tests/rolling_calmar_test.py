import numpy as np

from taflow import RollingCalmar


def test_matches_reference() -> None:
    values = np.array([1.0, 2.0, 1.0, 3.0, 2.0])
    actual = RollingCalmar(3).extend(values).compute()
    expected = np.full(values.size, np.nan)
    for index in range(2, values.size):
        window = values[index - 2 : index + 1]
        peak = np.maximum.accumulate(window)
        drawdown = np.min(np.where(peak != 0.0, window / peak - 1.0, 0.0))
        expected[index] = window.mean() / -drawdown if drawdown < 0.0 else 0.0
    np.testing.assert_allclose(actual, expected, equal_nan=True)
