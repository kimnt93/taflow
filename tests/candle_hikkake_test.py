import numpy as np

from taflow import CandleHikkake


def test_lifecycle_and_reset() -> None:
    values = np.arange(16.0) + 100.0
    indicator = CandleHikkake().extend(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator.compute()) == len(values)
    assert indicator.reset().value is None
