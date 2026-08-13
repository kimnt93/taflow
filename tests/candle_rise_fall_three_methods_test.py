import numpy as np

from taflow import CandleRiseFallThreeMethods


def test_rise_fall_three_methods_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleRiseFallThreeMethods().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

