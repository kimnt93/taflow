import numpy as np
import wickra

from taflow import VolumeRelativeStrengthIndex


def test_volume_relative_strength_index_matches_wickra_and_lifecycle() -> None:
    volume = np.array([10.0, 20.0, 15.0, 25.0, 20.0, 30.0, 30.0])
    # Wickra's Python batch boundary still accepts a close column, although
    # the native VolumeRsi formula uses only volume.
    expected = wickra.VolumeRsi(3).batch(np.zeros_like(volume), volume)
    batch = VolumeRelativeStrengthIndex(period=3).extend(volume)

    np.testing.assert_allclose(batch.compute(), expected, equal_nan=True)
    assert len(batch) == len(volume)
    assert batch.value == expected[-1]

    streamed = VolumeRelativeStrengthIndex(period=3)
    for value in volume:
        assert streamed.append(value) is streamed
    np.testing.assert_array_equal(streamed.compute(), batch.compute())

    assert streamed.reset() is streamed
    streamed.extend(volume[:2]).extend(volume[2:])
    np.testing.assert_array_equal(streamed.compute(), batch.compute())
