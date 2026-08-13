import numpy as np
import pytest
import wickra

from taflow import RelativeMomentumIndex


@pytest.mark.parametrize("timeperiod", [1, 2, 3, 14, 30])
@pytest.mark.parametrize("momentum", [1, 2, 5, 12])
@pytest.mark.parametrize("case", ["random", "constant", "monotonic", "repeated"])
def test_matches_wickra(timeperiod: int, momentum: int, case: str) -> None:
    rng = np.random.default_rng(82031 + timeperiod * 101 + momentum)
    size = 257
    if case == "random":
        close = 100.0 + rng.normal(0.0, 1.0, size).cumsum()
    elif case == "constant":
        close = np.full(size, 42.0)
    elif case == "monotonic":
        close = np.linspace(10.0, 90.0, size)
    else:
        close = np.resize(np.array([10.0, 12.0, 12.0, 9.0, 9.0, 12.0]), size)

    expected = np.asarray(wickra.RMI(timeperiod, momentum).batch(close.tolist()))
    actual = RelativeMomentumIndex(timeperiod, momentum).extend(close).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-12, equal_nan=True)


@pytest.mark.parametrize("timeperiod,momentum", [(1, 1), (3, 2), (14, 5), (30, 12)])
def test_minimum_length_matches_wickra(timeperiod: int, momentum: int) -> None:
    close = np.linspace(5.0, 5.0 + momentum + timeperiod, momentum + timeperiod)
    expected = np.asarray(wickra.RMI(timeperiod, momentum).batch(close.tolist()))
    actual = RelativeMomentumIndex(timeperiod, momentum).extend(close).compute()
    np.testing.assert_allclose(actual, expected, rtol=0.0, atol=2e-12, equal_nan=True)


def test_lifecycle_is_bitwise_invariant() -> None:
    rng = np.random.default_rng(21931)
    close = 100.0 + rng.normal(size=431).cumsum()
    batch = RelativeMomentumIndex(17, 5).extend(close)

    chunked = RelativeMomentumIndex(17, 5)
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


def test_warm_up_validation_and_input_contract() -> None:
    state = RelativeMomentumIndex(3, 2)
    for value in (1.0, 2.0, 3.0, 4.0):
        assert state.append(value) is state
        assert state.value is None
    assert state.append(5.0) is state
    assert state.value == 100.0
    np.testing.assert_array_equal(np.isnan(state.compute()), [True, True, True, True, False])

    with pytest.raises(ValueError):
        RelativeMomentumIndex().extend(None)
    with pytest.raises(ValueError):
        RelativeMomentumIndex().extend([[1.0, 2.0]])
    with pytest.raises(ValueError):
        RelativeMomentumIndex(0, 5)
    with pytest.raises(ValueError):
        RelativeMomentumIndex(14, 0)
