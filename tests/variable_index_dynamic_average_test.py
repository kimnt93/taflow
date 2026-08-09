import numpy as np
import pandas as pd
import pandas_ta_classic as pta
import pytest

from taflow import VariableIndexDynamicAverage


@pytest.mark.parametrize("length", [1, 2, 14, 30])
@pytest.mark.parametrize("case", ["random", "constant", "monotonic", "repeated"])
def test_matches_pandas_ta_classic(length: int, case: str) -> None:
    rng = np.random.default_rng(39071 + length)
    size = max(257, length)
    if case == "random":
        close = 100.0 + rng.normal(0.0, 1.0, size).cumsum()
    elif case == "constant":
        close = np.full(size, 42.0)
    elif case == "monotonic":
        close = np.linspace(10.0, 90.0, size)
    else:
        close = np.resize(np.array([10.0, 12.0, 12.0, 9.0, 9.0, 12.0]), size)

    expected = pta.vidya(pd.Series(close), length=length).to_numpy()
    actual = VariableIndexDynamicAverage(close, length).compute()
    np.testing.assert_allclose(
        actual, expected, rtol=1e-13, atol=1e-12, equal_nan=True
    )


@pytest.mark.parametrize("length", [1, 2, 14, 30])
def test_minimum_length_matches_pandas_ta_classic(length: int) -> None:
    close = np.linspace(5.0, 5.0 + length - 1, length)
    expected = pta.vidya(pd.Series(close), length=length).to_numpy()
    actual = VariableIndexDynamicAverage(close, length).compute()
    np.testing.assert_array_equal(actual, expected)


def test_lifecycle_is_bitwise_invariant_with_custom_alpha() -> None:
    rng = np.random.default_rng(94117)
    close = 100.0 + rng.normal(size=431).cumsum()
    batch = VariableIndexDynamicAverage(close, 17, 0.35)

    chunked = VariableIndexDynamicAverage([], 17, 0.35)
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


def test_warm_up_and_validation() -> None:
    state = VariableIndexDynamicAverage([], 3)
    assert state.append(1.0) is state
    assert state.value is None
    assert np.isnan(state.compute()[0])
    state.append(2.0).append(3.0)
    assert state.value == 2.0

    with pytest.raises(ValueError):
        VariableIndexDynamicAverage(None)
    with pytest.raises(ValueError):
        VariableIndexDynamicAverage([[1.0, 2.0]])
    with pytest.raises(ValueError):
        VariableIndexDynamicAverage([], 0)
    for invalid_alpha in (0.0, -0.1, 1.1, np.nan, np.inf):
        with pytest.raises(ValueError):
            VariableIndexDynamicAverage([], 14, invalid_alpha)
