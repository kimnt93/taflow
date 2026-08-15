import numpy as np
import pytest
import wickra

from taflow import RollingKendallRankCorrelation


def test_rolling_kendall_rank_correlation_matches_wickra_and_lifecycle() -> None:
    rng = np.random.default_rng(431)
    x = rng.integers(-5, 6, size=257).astype(np.float64)
    y = rng.integers(-7, 8, size=257).astype(np.float64)
    expected = np.asarray(wickra.KendallTau(20).batch(x, y))

    all_at_once = RollingKendallRankCorrelation(20).extend(x, y)
    np.testing.assert_array_equal(all_at_once.compute(), expected)

    chunked = RollingKendallRankCorrelation(20)
    for start in range(0, len(x), 7):
        chunked.extend(x[start : start + 7], y[start : start + 7])
    np.testing.assert_array_equal(chunked.compute(), expected)
    assert chunked.value == all_at_once.value

    scalar = RollingKendallRankCorrelation(20)
    for pair in zip(x, y):
        scalar.append(*pair)
    np.testing.assert_array_equal(scalar.compute(), expected)
    assert scalar.value == all_at_once.value

    before = scalar.compute().copy()
    with pytest.raises(ValueError):
        scalar.extend([1.0, 2.0], [3.0])
    np.testing.assert_array_equal(scalar.compute(), before)
    scalar.reset()
    assert len(scalar) == 0
    assert scalar.value is None
