"""Cornish-Fisher modified Sharpe ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import ModifiedSharpeRatio as _Native
from ._input import as_metric_series


class ModifiedSharpeRatio:
    """Compute mean excess return divided by Cornish-Fisher modified VaR.

    TAFlow freezes the single-output PerformanceAnalytics 2.1.0 convention
    represented by ``SharpeRatio.modified(..., FUN="VaR")`` with modified VaR,
    arithmetic return, and no annualization. The numerator is mean per-period
    excess return. The denominator uses population second through fourth
    central moments, moment skewness, excess kurtosis, and the second-order
    Cornish-Fisher expansion at ``confidence_level`` (default 0.95). As in the
    source, inverse risk is undefined and modified VaR above 100% is capped at
    one. The CRAN source tarball is pinned by SHA-256
    ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.
    No R runtime is installed, so tests translate that source and use Python's
    standard-normal quantile as the executable oracle.

    ``annual_risk_free_rate`` is an annual effective rate converted by Rust to
    the per-period ``Rf`` required by PerformanceAnalytics using explicit
    ``periods_per_year``. The ratio itself remains at input periodicity. Warm-up
    requires two usable returns; zero or inverse modified risk returns ``None``.
    PerformanceAnalytics technically evaluates singleton input, but TAFlow
    normalizes that degenerate higher-moment case to insufficient data.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L with positive initial capital. Rust owns all
    conversion and O(1) online moments. NaNs are omitted by default or rejected
    with ``nan_policy="raise"``; infinities and simple returns below -100% are
    rejected. Mutating lifecycle methods are fluent and bulk work releases the
    GIL.
    """

    def __init__(self, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, confidence_level: float = 0.95, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(periods_per_year), float(annual_risk_free_rate), float(confidence_level), nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "ModifiedSharpeRatio":
        """Append chronological returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None = None) -> "ModifiedSharpeRatio":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None = None) -> "ModifiedSharpeRatio":
        """Append chronological equity and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None = None) -> "ModifiedSharpeRatio":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> "ModifiedSharpeRatio":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ModifiedSharpeRatio":
        """Append chronological values in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the modified Sharpe ratio, or ``None`` until defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "ModifiedSharpeRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["ModifiedSharpeRatio"]
