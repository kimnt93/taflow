import numpy as np

from taflow import FracDiff


def test_frac_diff_lifecycle() -> None:
    values = np.arange(1.0, 129.0)
    state = FracDiff(threshold=1e-3).extend(values)
    first = state.compute()
    assert np.isnan(first).any() and np.isfinite(first).any()
    state.reset().extend(values)
    np.testing.assert_array_equal(state.compute(), first)
    assert len(state) == len(values)

