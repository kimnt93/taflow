import numpy as np

from taflow import VolumeWeightedMovingAverage


def test_weighted_average_and_validation() -> None:
    price = np.array([10.0, 20.0, 30.0])
    volume = np.array([1.0, 3.0, 2.0])
    actual = VolumeWeightedMovingAverage(price, volume, 2).compute()
    np.testing.assert_allclose(actual, [np.nan, 17.5, 24.0], equal_nan=True)
    try:
        VolumeWeightedMovingAverage(price, volume[:-1], 2)
    except ValueError:
        pass
    else:
        raise AssertionError("misaligned inputs must be rejected")
