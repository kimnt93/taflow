"""Sharpe significance adjusted for multiple strategy trials."""

from __future__ import annotations

from typing import Any

from .._native.metrics import DeflatedSharpeRatio as _Native
from ._input import as_metric_series


class DeflatedSharpeRatio:
    """Estimate Sharpe significance after selection across multiple trials.

    The oracle is vectorbt 0.28.5 commit
    ``993ceca7116fc8e55f4cd3a36fe43d83dab62b27``, implementing Bailey and
    Lopez de Prado. TAFlow requires explicit ``number_of_trials`` and
    ``annual_sharpe_ratio_variance`` across those trials; it never infers them
    from one observed return stream. Annual variance is divided by
    ``periods_per_year`` before vectorbt's expected-maximum-Sharpe approximation.
    Observed Sharpe uses sample deviation, while skew and Pearson kurtosis use
    SciPy-compatible bias-corrected sample estimators. Warm-up requires four
    usable normalized returns; constants and invalid variance adjustment return
    ``None``.

    Rust owns semantic input conversion and O(1) online moments through fourth
    order. ``append`` is allocation-free, ``compute`` is O(1), and bulk
    ``extend`` releases the GIL. Inputs support simple returns, log returns,
    positive equity levels, and period P&L with positive initial capital. NaNs
    are omitted by default or rejected with ``nan_policy="raise"``.
    """

    def __init__(self, number_of_trials: int, annual_sharpe_ratio_variance: float, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(number_of_trials, float(annual_sharpe_ratio_variance), float(periods_per_year), float(annual_risk_free_rate), nan_policy)

    def from_returns(self, returns: Any, *, column: str | None = None) -> "DeflatedSharpeRatio":
        """Append chronological returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(self, log_returns: Any, *, column: str | None = None) -> "DeflatedSharpeRatio":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(self, equity: Any, *, column: str | None = None) -> "DeflatedSharpeRatio":
        """Append chronological equity and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(self, pnl: Any, initial_capital: float, *, column: str | None = None) -> "DeflatedSharpeRatio":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(as_metric_series(pnl, column=column), float(initial_capital))
        return self

    def append(self, value: float) -> "DeflatedSharpeRatio":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value)); return self

    def extend(self, values: Any, *, column: str | None = None) -> "DeflatedSharpeRatio":
        """Append chronological values in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column)); return self

    @property
    def value(self) -> float | None:
        """Return the current deflated-Sharpe probability or ``None``."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the cached O(1) result without replaying prior input."""
        return self._state.compute()

    def reset(self) -> "DeflatedSharpeRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset(); return self

    def __len__(self) -> int:
        """Return the usable normalized-return count delegated to Rust."""
        return len(self._state)


__all__ = ["DeflatedSharpeRatio"]
