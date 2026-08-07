import numpy as np
import pytest

from taflow import RollingAutocorr, RollingEntropy


def test_rolling_entropy_and_autocorr_warmup_and_chunks():
    values = np.arange(1.0, 8.0)
    entropy = RollingEntropy(timeperiod=3).extend(values).compute()
    autocorr = RollingAutocorr(timeperiod=3).extend(values).compute()
    assert np.isnan(entropy[:2]).all()
    np.testing.assert_allclose(entropy[2:], np.log(3.0))
    np.testing.assert_allclose(autocorr[2:], [1.0, 1.0, 1.0, 1.0, 1.0])


def test_rolling_entropy_and_autocorr_reject_invalid_periods():
    with pytest.raises(ValueError): RollingEntropy(timeperiod=0)
    with pytest.raises(ValueError): RollingAutocorr(timeperiod=1)
