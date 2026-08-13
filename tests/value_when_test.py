import numpy as np

from taflow import ValueWhen


def test_carries_latest_true_value() -> None:
    condition = np.array([False, True, False])
    values = np.array([1.0, 2.0, 3.0])
    np.testing.assert_allclose(ValueWhen().extend(condition, values).compute(), [np.nan, 2.0, 2.0], equal_nan=True)
