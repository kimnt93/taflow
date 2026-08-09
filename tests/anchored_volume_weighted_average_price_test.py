import numpy as np
import pandas as pd
import pytest

from taflow import AnchoredVolumeWeightedAveragePrice


def pandas_grouped_weighted_moments(
    high: np.ndarray,
    low: np.ndarray,
    close: np.ndarray,
    volume: np.ndarray,
    anchor: np.ndarray,
    multiplier: float,
) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
    """Independent pandas groupby/cumulative-sum reference."""
    high_series = pd.Series(high)
    low_series = pd.Series(low)
    close_series = pd.Series(close)
    volume_series = pd.Series(volume)
    groups = pd.Series(anchor, dtype=bool).cumsum()
    typical = (high_series + low_series + close_series) / 3.0
    cumulative_volume = volume_series.groupby(groups, sort=False).cumsum()
    cumulative_weighted = (typical * volume_series).groupby(
        groups, sort=False
    ).cumsum()
    cumulative_weighted_square = (typical * typical * volume_series).groupby(
        groups, sort=False
    ).cumsum()
    average = cumulative_weighted / cumulative_volume
    variance = (cumulative_weighted_square / cumulative_volume - average * average).clip(
        lower=0.0
    )
    deviation = variance.pow(0.5) * multiplier
    return (
        average.to_numpy(),
        (average + deviation).to_numpy(),
        (average - deviation).to_numpy(),
    )


@pytest.mark.parametrize("multiplier", [0.0, 1.0, 2.5])
@pytest.mark.parametrize("case", ["random", "constant", "monotonic", "minimum"])
def test_matches_pandas_grouped_weighted_moments(
    multiplier: float, case: str
) -> None:
    rng = np.random.default_rng(72841)
    size = 257 if case != "minimum" else 1
    if case == "random":
        close = 100.0 + rng.normal(0.0, 1.0, size).cumsum()
    elif case == "constant":
        close = np.full(size, 37.5)
    elif case == "monotonic":
        close = np.linspace(10.0, 110.0, size)
    else:
        close = np.array([12.0])
    high = close + rng.uniform(0.0, 2.0, size)
    low = close - rng.uniform(0.0, 2.0, size)
    volume = rng.uniform(1.0, 1_000.0, size)
    anchor = np.zeros(size, dtype=bool)
    anchor[::31] = True
    if size > 80:
        anchor[62:65] = True

    expected = pandas_grouped_weighted_moments(
        high, low, close, volume, anchor, multiplier
    )
    actual = AnchoredVolumeWeightedAveragePrice(
        high, low, close, volume, anchor, multiplier
    ).compute()
    for actual_output, expected_output in zip(actual, expected, strict=True):
        np.testing.assert_allclose(
            actual_output, expected_output, rtol=1e-12, atol=1e-9, equal_nan=True
        )


def test_lifecycle_is_bitwise_invariant() -> None:
    rng = np.random.default_rng(61577)
    close = 50.0 + rng.normal(size=211).cumsum()
    high = close + rng.uniform(0.1, 1.5, len(close))
    low = close - rng.uniform(0.1, 1.5, len(close))
    volume = rng.uniform(1.0, 500.0, len(close))
    anchor = np.zeros(len(close), dtype=bool)
    anchor[[0, 43, 44, 109, 170]] = True

    batch = AnchoredVolumeWeightedAveragePrice(
        high, low, close, volume, anchor, 1.75
    )
    chunked = AnchoredVolumeWeightedAveragePrice([], [], [], [], [], 1.75)
    assert (
        chunked.extend(
            high[:47], low[:47], close[:47], volume[:47], anchor[:47]
        )
        is chunked
    )
    assert (
        chunked.extend(
            high[47:], low[47:], close[47:], volume[47:], anchor[47:]
        )
        is chunked
    )
    for chunked_output, batch_output in zip(
        chunked.compute(), batch.compute(), strict=True
    ):
        np.testing.assert_array_equal(chunked_output, batch_output)

    assert chunked.reset() is chunked
    assert chunked.value is None
    for bar in zip(high, low, close, volume, anchor, strict=True):
        assert chunked.append(*bar) is chunked
    for replay_output, batch_output in zip(
        chunked.compute(), batch.compute(), strict=True
    ):
        np.testing.assert_array_equal(replay_output, batch_output)
    assert chunked.value == batch.value
    assert len(chunked) == len(close)


def test_validates_configuration_and_alignment_before_mutation() -> None:
    for invalid in (-1.0, np.nan, np.inf):
        with pytest.raises(ValueError):
            AnchoredVolumeWeightedAveragePrice([], [], [], [], [], invalid)
    with pytest.raises(ValueError):
        AnchoredVolumeWeightedAveragePrice(None, [], [], [], [])

    state = AnchoredVolumeWeightedAveragePrice([], [], [], [], [])
    state.append(2.0, 0.0, 1.0, 10.0, True)
    prior_value = state.value
    with pytest.raises(ValueError):
        state.extend([2.0], [], [1.0], [10.0], [False])
    assert len(state) == 1
    assert state.value == prior_value


def test_zero_volume_outputs_nan_until_positive_cumulative_volume() -> None:
    state = AnchoredVolumeWeightedAveragePrice(
        [2.0, 3.0], [0.0, 1.0], [1.0, 2.0], [0.0, 5.0], [True, False]
    )
    average, upper, lower = state.compute()
    assert np.isnan(average[0]) and np.isnan(upper[0]) and np.isnan(lower[0])
    np.testing.assert_array_equal(
        [average[1], upper[1], lower[1]], [2.0, 2.0, 2.0]
    )
