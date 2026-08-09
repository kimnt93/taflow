import numpy as np
import pandas as pd
import pandas_ta_classic as pta
import pytest

from taflow import LaguerreRelativeStrengthIndex


@pytest.mark.parametrize("gamma", [0.1, 0.25, 0.5, 0.9])
@pytest.mark.parametrize("case", ["random", "constant", "monotonic", "repeated"])
def test_matches_pandas_ta_classic(gamma: float, case: str) -> None:
    rng = np.random.default_rng(13691 + int(gamma * 100))
    size = 257
    if case == "random":
        close = 100.0 + rng.normal(0.0, 1.0, size).cumsum()
    elif case == "constant":
        close = np.full(size, 42.0)
    elif case == "monotonic":
        close = np.linspace(10.0, 90.0, size)
    else:
        close = np.resize(np.array([10.0, 12.0, 12.0, 9.0, 9.0, 12.0]), size)

    expected = pta.lrsi(pd.Series(close), length=1, gamma=gamma).to_numpy()
    actual = LaguerreRelativeStrengthIndex(close, gamma).compute()
    np.testing.assert_allclose(
        actual, expected, rtol=1e-13, atol=1e-12, equal_nan=True
    )


@pytest.mark.parametrize("gamma", [0.1, 0.5, 0.9])
def test_minimum_length_matches_pandas_ta_classic(gamma: float) -> None:
    close = np.array([17.0])
    expected = pta.lrsi(pd.Series(close), length=1, gamma=gamma).to_numpy()
    actual = LaguerreRelativeStrengthIndex(close, gamma).compute()
    np.testing.assert_array_equal(actual, expected)


def test_lifecycle_is_bitwise_invariant() -> None:
    rng = np.random.default_rng(52519)
    close = 100.0 + rng.normal(size=431).cumsum()
    batch = LaguerreRelativeStrengthIndex(close, 0.35)

    chunked = LaguerreRelativeStrengthIndex([], 0.35)
    assert chunked.extend(close[:53]) is chunked
    assert chunked.extend(close[53:]) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert chunked.value == batch.value

    assert chunked.reset() is chunked
    assert chunked.value is None
    for value in close:
        assert chunked.append(float(value)) is chunked
    np.testing.assert_array_equal(chunked.compute(), batch.compute())
    assert chunked.value == batch.value
    assert len(chunked) == len(close)


def test_validation_and_input_contract() -> None:
    with pytest.raises(ValueError):
        LaguerreRelativeStrengthIndex(None)
    with pytest.raises(ValueError):
        LaguerreRelativeStrengthIndex([[1.0, 2.0]])
    for invalid_gamma in (-0.1, 1.0, 1.1, np.nan, np.inf):
        with pytest.raises(ValueError):
            LaguerreRelativeStrengthIndex([], invalid_gamma)

    state = LaguerreRelativeStrengthIndex([], 0.0)
    assert state.value is None
    assert state.append(42.0) is state
    assert state.value == 0.0
