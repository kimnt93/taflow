import numpy as np
import pytest

from taflow import FibonacciProjection


def test_fibonacci_projection_lifecycle_and_alignment() -> None:
    length = 5
    high = np.arange(10.0, 10.0 + length)
    low = np.arange(5.0, 5.0 + length)
    indicator = FibonacciProjection(high, low)

    outputs = indicator.compute()
    assert len(outputs) == 4
    assert len(indicator) == length
    for output in outputs:
        assert np.isnan(output[:2]).all()
    assert indicator.value is not None

    replay = tuple(output.copy() for output in outputs)
    indicator.reset().extend(high, low)
    for actual, expected in zip(indicator.compute(), replay, strict=True):
        np.testing.assert_array_equal(actual, expected)

    with pytest.raises(ValueError):
        FibonacciProjection([1.0], [0.0, 1.0])
