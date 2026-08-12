from __future__ import annotations
import numpy as np
import pytest
from taflow.metrics import MetricPipeline, SharpeRatio, SortinoRatio

def test_pipeline_computes_configured_metrics_under_caller_names() -> None:
    returns = np.array([0.01, -0.02, 0.03, 0.0, 0.04])
    pipeline = MetricPipeline()
    assert pipeline.add('sharpe', SharpeRatio()) is pipeline
    assert pipeline.add('sor', SortinoRatio()) is pipeline
    assert pipeline.from_returns(returns) is pipeline
    values = pipeline.compute()
    assert pipeline.metrics == ('sharpe', 'sor')
    assert values['sharpe'] == pytest.approx(SharpeRatio().from_returns(returns).compute())
    assert values['sor'] == pytest.approx(SortinoRatio().from_returns(returns).compute())

def test_pipeline_scalar_chunk_reset_and_domains_are_invariant() -> None:
    returns = np.array([0.1, -0.2, 0.05])
    batch = MetricPipeline().add('sharpe', SharpeRatio(12.0, 0.04))
    batch.from_returns(returns)
    scalar = MetricPipeline().add('sharpe', SharpeRatio(12.0, 0.04))
    scalar.from_returns([]).append(returns[0]).extend(returns[1:])
    assert scalar.compute() == pytest.approx(batch.compute())
    assert scalar.value == pytest.approx(batch.compute())
    assert scalar.reset().extend(returns).compute() == pytest.approx(batch.compute())
    log_returns = MetricPipeline().add('sharpe', SharpeRatio(12.0, 0.04))
    assert log_returns.from_log_returns(np.log1p(returns)).compute() == pytest.approx(batch.compute())
    equity = MetricPipeline().add('sharpe', SharpeRatio(12.0, 0.04))
    assert equity.from_equity([100.0, 110.0, 88.0, 92.4]).compute() == pytest.approx(batch.compute())
    pnl = MetricPipeline().add('sharpe', SharpeRatio(12.0, 0.04))
    assert pnl.from_pnl([10.0, -22.0, 4.4], 100.0).compute() == pytest.approx(batch.compute())

def test_pipeline_validation() -> None:
    pipeline = MetricPipeline().add('sharpe', SharpeRatio())
    with pytest.raises(ValueError):
        pipeline.add('sharpe', SortinoRatio())
    with pytest.raises(ValueError):
        pipeline.append(0.01)
    pipeline.from_returns([0.01])
    with pytest.raises(ValueError):
        pipeline.from_equity([100.0])
    with pytest.raises(ValueError):
        pipeline.add('late', SortinoRatio())
