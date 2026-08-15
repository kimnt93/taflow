import numpy as np
import talib

from taflow import HilbertTransformPhasor


def _assert_lifecycle(values: np.ndarray) -> None:
    expected = talib.HT_PHASOR(values)
    full = HilbertTransformPhasor().extend(values)
    for actual, wanted in zip(full.compute(), expected):
        np.testing.assert_array_equal(actual, wanted)

    chunked = HilbertTransformPhasor()
    for chunk in np.array_split(values, 7):
        assert chunked.extend(chunk) is chunked
    for actual, wanted in zip(chunked.compute(), expected):
        np.testing.assert_array_equal(actual, wanted)
    assert chunked.value == full.value

    assert chunked.reset() is chunked
    assert chunked.value is None
    for value in values:
        assert chunked.append(float(value)) is chunked
    for actual, wanted in zip(chunked.compute(), expected):
        np.testing.assert_array_equal(actual, wanted)


def test_hilbert_transform_phasor_matches_talib_and_lifecycle():
    rng = np.random.default_rng(7392)
    _assert_lifecycle(100.0 + np.cumsum(rng.normal(0.0, 0.4, 257)))
    _assert_lifecycle(np.full(96, 12.5))
    _assert_lifecycle(np.sin(np.linspace(0.0, 15.0, 129)))
