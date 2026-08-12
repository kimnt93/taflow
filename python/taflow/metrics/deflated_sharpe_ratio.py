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
    positive equity levels, and period P&L with positive initial equity. NaNs
    are omitted by default or rejected with ``nan_policy="raise"``.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use DeflatedSharpeRatio.from_returns/from_equity/from_pnl/from_log_returns")

    @classmethod
    def _create(cls, values: Any, input_mode: str, *, number_of_trials: int, annual_sharpe_ratio_variance: float, periods_per_year: float, annual_risk_free_rate: float, initial_equity: float | None, nan_policy: str, column: str | None) -> "DeflatedSharpeRatio":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, int(number_of_trials), float(annual_sharpe_ratio_variance), float(periods_per_year), float(annual_risk_free_rate), initial_equity, nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_returns(cls, returns: Any, *, number_of_trials: int, annual_sharpe_ratio_variance: float, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, nan_policy: str = "omit", column: str | None = None) -> "DeflatedSharpeRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", number_of_trials=number_of_trials, annual_sharpe_ratio_variance=annual_sharpe_ratio_variance, periods_per_year=periods_per_year, annual_risk_free_rate=annual_risk_free_rate, initial_equity=None, nan_policy=nan_policy, column=column)

    @classmethod
    def from_log_returns(cls, log_returns: Any, *, number_of_trials: int, annual_sharpe_ratio_variance: float, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, nan_policy: str = "omit", column: str | None = None) -> "DeflatedSharpeRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", number_of_trials=number_of_trials, annual_sharpe_ratio_variance=annual_sharpe_ratio_variance, periods_per_year=periods_per_year, annual_risk_free_rate=annual_risk_free_rate, initial_equity=None, nan_policy=nan_policy, column=column)

    @classmethod
    def from_equity(cls, equity: Any, *, number_of_trials: int, annual_sharpe_ratio_variance: float, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, nan_policy: str = "omit", column: str | None = None) -> "DeflatedSharpeRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", number_of_trials=number_of_trials, annual_sharpe_ratio_variance=annual_sharpe_ratio_variance, periods_per_year=periods_per_year, annual_risk_free_rate=annual_risk_free_rate, initial_equity=None, nan_policy=nan_policy, column=column)

    @classmethod
    def from_pnl(cls, pnl: Any, *, initial_equity: float, number_of_trials: int, annual_sharpe_ratio_variance: float, periods_per_year: float = 252.0, annual_risk_free_rate: float = 0.0, nan_policy: str = "omit", column: str | None = None) -> "DeflatedSharpeRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(pnl, "pnl", number_of_trials=number_of_trials, annual_sharpe_ratio_variance=annual_sharpe_ratio_variance, periods_per_year=periods_per_year, annual_risk_free_rate=annual_risk_free_rate, initial_equity=float(initial_equity), nan_policy=nan_policy, column=column)

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
