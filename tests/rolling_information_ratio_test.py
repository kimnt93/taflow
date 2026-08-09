import numpy as np

from taflow import RollingInformationRatio


def test_rolling_information_ratio_lifecycle():
    values = np.linspace(1.0, 2.0, 80)
    benchmark = values * 0.9
    indicator = RollingInformationRatio(values, benchmark, timeperiod=10)
    output = indicator.compute()
    assert output.shape == values.shape
    assert len(indicator) == len(values)
    indicator.reset().extend(values[:20], benchmark[:20])
    assert len(indicator) == 20


def test_rolling_information_ratio_rejects_misaligned_inputs():
    with np.testing.assert_raises(ValueError):
        RollingInformationRatio([1, 2], [1])
