import numpy as np
import pytest

from taflow import RollingAlpha, RollingInformationRatio


def test_rolling_alpha_and_information_ratio():
    benchmark = np.arange(1.0, 8.0)
    input = benchmark + 3.0
    alpha = RollingAlpha(timeperiod=3).extend(input, benchmark).compute()
    ratio = RollingInformationRatio(timeperiod=3).extend(input, benchmark).compute()
    np.testing.assert_allclose(alpha[2:], 3.0)
    np.testing.assert_allclose(ratio[2:], 0.0)


def test_rolling_alpha_rejects_mismatched_inputs():
    with pytest.raises(ValueError): RollingAlpha(timeperiod=0)
    with pytest.raises(ValueError): RollingAlpha().extend(np.ones(2), np.ones(1))
