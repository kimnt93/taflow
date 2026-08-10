import numpy as np
import wickra

from taflow import VolumeZoneOscillator


def test_volume_zone_oscillator_matches_wickra_and_lifecycle() -> None:
    close = np.array([10.0, 11.0, 10.0, 12.0, 11.0, 13.0, 12.0])
    volume = np.array([100.0, 110.0, 120.0, 130.0, 140.0, 150.0, 160.0])
    expected = wickra.VZO(3).batch(close, volume)

    batch = VolumeZoneOscillator(close, volume, timeperiod=3)
    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(close)
    assert batch.value == expected[-1]

    chunked = VolumeZoneOscillator(
        np.array([], dtype=float), np.array([], dtype=float), timeperiod=3
    )
    chunked.extend(close[:2], volume[:2]).extend(close[2:], volume[2:])
    np.testing.assert_array_equal(chunked.compute(), batch.compute())

    assert chunked.reset() is chunked
    for bar in zip(close, volume, strict=True):
        assert chunked.append(*bar) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())


def test_volume_zone_oscillator_rejects_misaligned_input() -> None:
    with np.testing.assert_raises(ValueError):
        VolumeZoneOscillator([1.0, 2.0], [10.0], timeperiod=3)
