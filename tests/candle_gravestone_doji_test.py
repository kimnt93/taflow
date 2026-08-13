import numpy as np

from taflow import CandleGravestoneDoji


def test_gravestone_doji_lifecycle():
    values = np.linspace(100.0, 110.0, 20)
    indicator = CandleGravestoneDoji().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None

