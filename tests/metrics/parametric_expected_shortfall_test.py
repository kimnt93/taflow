import numpy as np
import pytest
from scipy.stats import norm
from taflow.metrics import ParametricExpectedShortfall

@pytest.mark.parametrize(('values', 'cutoff'), [(np.array([-0.02, 0.01, 0.03, -0.01, 0.015]), 0.05), (np.linspace(-0.04, 0.05, 101), 0.01), (np.random.default_rng(20260812).normal(0.0004, 0.012, 513), 0.1), (np.array([np.nan, 0.02, -0.03, np.nan, 0.01]), 0.025)])
def test_matches_scipy_gaussian_expected_shortfall(values, cutoff):
    usable = values[~np.isnan(values)]
    expected = float(usable.mean() - usable.std(ddof=1) * norm.pdf(norm.ppf(cutoff)) / cutoff)
    actual = ParametricExpectedShortfall(cutoff=cutoff).from_returns(values).compute()
    assert actual == pytest.approx(expected, rel=2e-08, abs=2e-10)

def test_input_methods_and_streaming_lifecycle():
    returns = np.array([0.01, -0.02, 0.03, -0.015])
    equity = 100.0 * np.cumprod(np.r_[1.0, 1.0 + returns])
    expected = ParametricExpectedShortfall().from_returns(returns).compute()
    assert ParametricExpectedShortfall().from_log_returns(np.log1p(returns)).compute() == pytest.approx(expected)
    assert ParametricExpectedShortfall().from_equity(equity).compute() == pytest.approx(expected)
    assert ParametricExpectedShortfall().from_pnl(np.diff(equity), initial_capital=100.0).compute() == pytest.approx(expected)
    state = ParametricExpectedShortfall().from_returns([])
    assert state.append(returns[0]).compute() is None
    assert state.extend(returns[1:]).compute() == pytest.approx(expected)
    assert state.reset().extend(returns).compute() == pytest.approx(expected)

def test_validation_and_constant_edge():
    unbound = ParametricExpectedShortfall()
    with pytest.raises(ValueError):
        unbound.append(0.01)
    assert ParametricExpectedShortfall().from_returns([0.01]).compute() is None
    assert ParametricExpectedShortfall().from_returns([0.01, 0.01]).compute() == pytest.approx(0.01)
    with pytest.raises(ValueError):
        ParametricExpectedShortfall(cutoff=0.0).from_returns([0.01, -0.01])
    with pytest.raises(ValueError):
        ParametricExpectedShortfall().from_returns([np.inf])
