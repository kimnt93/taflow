import numpy as np

from taflow import CumulativeCount


def test_count_and_reset_lifecycle() -> None:
    values = np.arange(8, dtype=np.float64)
    indicator = CumulativeCount(values)
    np.testing.assert_array_equal(indicator.compute(), np.arange(1.0, 9.0))
    indicator.reset().append(0.0)
    assert indicator.value == 1.0
