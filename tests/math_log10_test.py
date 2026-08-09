import numpy as np
import talib

from taflow import MathLog10


def test_math_log10_matches_talib_and_lifecycle() -> None:
    rng = np.random.default_rng(104729)
    primary = np.linspace(0.1, 12.0, 127)
    datasets = (
        primary[:1],
        primary,
        np.full(17, primary[len(primary) // 2]),
        rng.choice(primary, size=211, replace=True),
    )
    for values in datasets:
        expected = (talib.LOG10)(values)
        actual = MathLog10(values)
        np.testing.assert_allclose(
            actual.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )

        state = MathLog10([])
        split = len(values) // 3
        assert state.extend(values[:split]) is state
        assert state.extend(values[split:]) is state
        np.testing.assert_allclose(
            state.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )
        assert state.value == state.compute()[-1]
        assert state.reset() is state
        for value in values:
            assert state.append(float(value)) is state
        np.testing.assert_allclose(
            state.compute(), expected, rtol=1e-12, atol=1e-12, equal_nan=True
        )

    fresh = MathLog10([])
    assert len(fresh) == 0
    assert fresh.value is None
