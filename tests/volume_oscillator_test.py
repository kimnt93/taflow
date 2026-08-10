import numpy as np
import wickra

from taflow import VolumeOscillator


def test_volume_oscillator_matches_wickra_and_lifecycle() -> None:
    volume = np.array([100.0, 110.0, 120.0, 130.0, 140.0, 150.0])
    expected = wickra.VolumeOscillator(2, 4).batch(volume)

    batch = VolumeOscillator(volume, fast=2, slow=4)
    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(volume)
    assert batch.value == expected[-1]

    chunked = VolumeOscillator(np.array([], dtype=float), fast=2, slow=4)
    assert chunked.extend(volume[:3]) is chunked
    assert chunked.extend(volume[3:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())

    assert chunked.reset() is chunked
    assert len(chunked) == 0
    for value in volume:
        assert chunked.append(value) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())


def test_volume_oscillator_rejects_invalid_period_order() -> None:
    with np.testing.assert_raises(ValueError):
        VolumeOscillator(np.array([], dtype=float), fast=4, slow=4)
