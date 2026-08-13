import numpy as np
import wickra

from taflow import KlingerVolumeOscillator


def test_klinger_volume_oscillator_matches_wickra():
    rng = np.random.default_rng(29)
    close = 100.0 + np.cumsum(rng.normal(size=256))
    high = close + rng.uniform(0.2, 1.5, len(close))
    low = close - rng.uniform(0.2, 1.5, len(close))
    volume = rng.uniform(100.0, 1000.0, len(close))
    actual = KlingerVolumeOscillator().extend(high, low, close, volume).compute()
    expected = np.asarray(
        wickra.KVO().batch(high, low, close, volume), dtype=np.float64
    )

    # Wickra exposes the oscillator only. TAFlow's signal line is an additional
    # native lifecycle output and is covered by chunk/reset invariance below.
    np.testing.assert_allclose(actual[0], expected, equal_nan=True, atol=1e-10)


def test_klinger_volume_oscillator_chunked_reset():
    close = np.linspace(90.0, 130.0, 80)
    high = close + 1.0
    low = close - 1.0
    volume = np.full_like(close, 500.0)
    whole = KlingerVolumeOscillator(5, 8, 3).extend(high, low, close, volume)
    chunked = KlingerVolumeOscillator(5, 8, 3)
    chunked.extend(high[:31], low[:31], close[:31], volume[:31])
    chunked.extend(high[31:], low[31:], close[31:], volume[31:])
    for left, right in zip(whole.compute(), chunked.compute()):
        np.testing.assert_array_equal(left, right)
    assert chunked.reset() is chunked
    assert len(chunked) == 0
