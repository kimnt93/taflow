import numpy as np
import pytest

from taflow import Fvg


def test_fvg_detects_and_reports_directional_mitigation():
    open_ = np.array([1.0, 1.0, 1.0, 1.0])
    high = np.array([10.0, 11.0, 13.0, 14.0])
    low = np.array([0.0, 1.0, 12.0, 9.0])
    close = np.array([1.0, 2.0, 1.0, 1.0])
    signal, top, bottom, mitigated = Fvg().extend(open_, high, low, close).compute()
    np.testing.assert_array_equal(signal, [np.nan, np.nan, 1.0, np.nan])
    np.testing.assert_array_equal(top, [np.nan, np.nan, 12.0, np.nan])
    np.testing.assert_array_equal(bottom, [np.nan, np.nan, 10.0, np.nan])
    np.testing.assert_array_equal(mitigated, [np.nan, np.nan, np.nan, 1.0])


def test_fvg_rejects_mismatched_inputs_and_resets():
    with pytest.raises(ValueError):
        Fvg().extend(np.ones(2), np.ones(2), np.ones(1), np.ones(2))
    state = Fvg().extend(np.ones(2), np.ones(2), np.ones(2), np.ones(2))
    state.reset()
    assert all(len(values) == 0 for values in state.compute())
