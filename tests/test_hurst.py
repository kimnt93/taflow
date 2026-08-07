import numpy as np
import pytest

from taflow import FractalDimension, Hurst


def test_hurst_family_is_aligned_and_chunk_invariant():
    values = np.arange(1.0, 10.0)
    hurst = Hurst(timeperiod=4).extend(values).compute()
    dimension = FractalDimension(timeperiod=4).extend(values).compute()
    assert np.isnan(hurst[:3]).all()
    np.testing.assert_allclose(hurst[3:] + dimension[3:], 2.0)
    chunked = Hurst(timeperiod=4)
    chunked.extend(values[:5]).extend(values[5:])
    np.testing.assert_array_equal(chunked.compute(), hurst)


def test_hurst_family_rejects_short_period():
    with pytest.raises(ValueError): Hurst(timeperiod=1)
