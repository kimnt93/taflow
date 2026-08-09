import numpy as np

from taflow import CumulativeSumControlChart


def test_cumulative_sum_control_chart_lifecycle() -> None:
    state = CumulativeSumControlChart(np.array([0.5, -0.5, 2.0, -1.0]), threshold=1.0)
    expected = np.array([0.0, 0.0, 1.0, 0.0])
    np.testing.assert_array_equal(state.compute(), expected)
    state.reset().extend(np.array([0.5, -0.5, 2.0, -1.0]))
    np.testing.assert_array_equal(state.compute(), expected)
    assert len(state) == 4

