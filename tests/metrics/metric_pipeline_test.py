from __future__ import annotations

import numpy as np
import pytest

from taflow.metrics import MaximumDrawdown, MetricPipeline, SharpeRatio, TotalReturn


def test_pnl_pipeline_matches_standalone_public_classes() -> None:
    pnl = np.array([10.0, -5.0, 7.0, -2.0, 3.0])
    selected = ("TotalReturn", "SharpeRatio", "MaximumDrawdown")
    actual = MetricPipeline.from_pnl(
        pnl, initial_equity=100.0, metrics=selected
    )
    expected = {
        "TotalReturn": TotalReturn.from_pnl(pnl, initial_equity=100.0).compute(),
        "SharpeRatio": SharpeRatio.from_pnl(pnl, initial_equity=100.0).compute(),
        "MaximumDrawdown": MaximumDrawdown.from_pnl(
            pnl, initial_equity=100.0
        ).compute(),
    }
    assert actual.metrics == selected
    assert actual.compute() == pytest.approx(expected)


def test_pipeline_scalar_chunk_reset_and_domain_factories_match() -> None:
    returns = np.array([0.01, -0.02, 0.03, 0.0, 0.04])
    selected = ("TotalReturn", "WinRate", "TailRatio")
    batch = MetricPipeline.from_returns(returns, metrics=selected)
    scalar = MetricPipeline.from_returns([], metrics=selected)
    assert scalar.extend(returns[:2]) is scalar
    for value in returns[2:]:
        assert scalar.append(value) is scalar
    assert scalar.compute() == pytest.approx(batch.compute())
    assert scalar.value == pytest.approx(batch.compute())
    assert scalar.reset() is scalar
    assert scalar.extend(returns).compute() == pytest.approx(batch.compute())
    assert MetricPipeline.from_log_returns(
        np.log1p(returns), metrics=selected
    ).compute() == pytest.approx(batch.compute())
    equity = 100.0 * np.cumprod(1.0 + returns)
    assert MetricPipeline.from_equity(
        np.r_[100.0, equity], metrics=selected
    ).compute() == pytest.approx(batch.compute())


def test_default_suite_and_validation() -> None:
    state = MetricPipeline.from_returns([0.01, -0.02, 0.03])
    assert tuple(state.compute()) == MetricPipeline.supported_metrics()
    with pytest.raises(ValueError):
        MetricPipeline.from_returns([0.01], metrics=["TotalReturn", "TotalReturn"])
    with pytest.raises(ValueError):
        MetricPipeline.from_returns([0.01], metrics=["TrackingError"])
    with pytest.raises(TypeError):
        MetricPipeline()
