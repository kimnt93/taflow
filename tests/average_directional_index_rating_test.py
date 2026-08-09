import numpy as np

from taflow import AverageDirectionalIndexRating


def test_average_directional_index_rating_lifecycle():
    close = np.linspace(100.0, 110.0, 60)
    indicator = AverageDirectionalIndexRating(close + 1.0, close - 1.0, close)
    assert len(indicator) == len(close)
    indicator.reset()
    assert indicator.value is None
