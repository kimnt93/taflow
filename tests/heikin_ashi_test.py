import numpy as np
import pandas as pd
import pandas_ta_classic as pta
import pytest

from taflow import HeikinAshi


def test_heikin_ashi_matches_pandas_ta_classic_and_lifecycle() -> None:
    rng = np.random.default_rng(90127)
    close = 100.0 + rng.normal(0.0, 1.0, 257).cumsum()
    _open = close + rng.normal(0.0, 0.4, 257)
    high = np.maximum(_open, close) + rng.uniform(0.1, 1.5, 257)
    low = np.minimum(_open, close) - rng.uniform(0.1, 1.5, 257)
    frame = pta.ha(
        pd.Series(_open), pd.Series(high), pd.Series(low), pd.Series(close)
    )
    expected = tuple(frame[column].to_numpy() for column in frame.columns)

    actual = HeikinAshi().extend(_open, high, low, close)
    for actual_output, expected_output in zip(actual.compute(), expected, strict=True):
        np.testing.assert_array_equal(actual_output, expected_output)

    chunked = HeikinAshi()
    assert chunked.extend(_open[:43], high[:43], low[:43], close[:43]) is chunked
    assert chunked.extend(_open[43:], high[43:], low[43:], close[43:]) is chunked
    for chunked_output, batch_output in zip(
        chunked.compute(), actual.compute(), strict=True
    ):
        np.testing.assert_array_equal(chunked_output, batch_output)
    assert chunked.reset() is chunked
    for bar in zip(_open, high, low, close, strict=True):
        assert chunked.append(*map(float, bar)) is chunked
    for replay_output, batch_output in zip(
        chunked.compute(), actual.compute(), strict=True
    ):
        np.testing.assert_array_equal(replay_output, batch_output)
    assert chunked.value == actual.value
    assert len(chunked) == len(close)


def test_heikin_ashi_requires_aligned_histories() -> None:
    with pytest.raises(ValueError):
        HeikinAshi().extend(None, [], [], [])
    with pytest.raises(ValueError):
        HeikinAshi().extend([1.0], [], [1.0], [1.0])
