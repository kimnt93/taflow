import numpy as np

from taflow import CandleThreeLineStrike


def test_lifecycle_and_reset() -> None:
    values = np.arange(48.0) + 100.0
    indicator = CandleThreeLineStrike(values, values + 2.0, values - 2.0, values + 0.5)
    assert len(indicator.compute()) == len(values)
    assert indicator.reset().value is None
