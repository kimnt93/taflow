"""Probability that a strategy Sharpe ratio exceeds a benchmark."""

from __future__ import annotations

from typing import Any

from .._native.metrics import ProbabilisticSharpeRatio as _Native
from ._input import as_metric_series


class ProbabilisticSharpeRatio:
    """Estimate the probability that Sharpe exceeds a benchmark Sharpe.

    TAFlow freezes the Bailey and Lopez de Prado probabilistic-Sharpe formula
    used by vectorbt 0.28.5's deflated-Sharpe kernel, with its estimated maximum
    Sharpe replaced by ``annual_benchmark_sharpe_ratio``. The pinned vectorbt
    source commit is ``993ceca7116fc8e55f4cd3a36fe43d83dab62b27`` and is
    the independent oracle.
    The estimated Sharpe uses sample standard deviation; skewness and Pearson
    kurtosis use SciPy-compatible bias-corrected sample estimators
    (``bias=False``). Both estimated and benchmark Sharpe are converted to
    per-period scale before evaluation. Warm-up requires four usable returns.

    ``annual_risk_free_rate`` is an annual effective rate converted by Rust to
    a per-period return using explicit ``periods_per_year``. Inputs may be
    decimal simple returns, log returns, positive equity levels, or
    non-cumulative period P&L with positive initial capital. NaNs are omitted by
    default or rejected with ``nan_policy="raise"``; infinities and simple
    returns below -100% are rejected. Rust owns conversion and O(1) online
    moments through fourth order. ``append`` is allocation-free, ``compute`` is
    O(1), mutating methods are fluent, and bulk work releases the GIL.
    """

    def __init__(self, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, annual_benchmark_sharpe_ratio: float = 0.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), float(annual_risk_free_rate), float(annual_benchmark_sharpe_ratio), nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "ProbabilisticSharpeRatio":
        """Append chronological returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None = None) -> "ProbabilisticSharpeRatio":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None = None) -> "ProbabilisticSharpeRatio":
        """Append chronological equity and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None = None) -> "ProbabilisticSharpeRatio":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> "ProbabilisticSharpeRatio":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ProbabilisticSharpeRatio":
        """Append chronological values in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current exceedance probability, or ``None`` if undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the cached O(1) result without replaying prior input."""
        return self._state.compute()

    def reset(self) -> "ProbabilisticSharpeRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["ProbabilisticSharpeRatio"]
