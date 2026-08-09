import numpy as np

from taflow import TripleExponentialRateOfChange


def test_triple_exponential_rate_of_change_lifecycle():
    values = np.linspace(100.0, 110.0, 100)
    indicator = TripleExponentialRateOfChange(values, timeperiod=7)
    assert len(indicator) == len(values)
    indicator.reset()
    assert indicator.value is None
