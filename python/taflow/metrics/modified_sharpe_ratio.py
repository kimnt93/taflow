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
    or non-cumulative period P&L with positive initial equity. Rust owns all
    conversion and O(1) online moments. NaNs are omitted by default or rejected
    with ``nan_policy="raise"``; infinities and simple returns below -100% are
    rejected. Mutating lifecycle methods are fluent and bulk work releases the
    GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use ModifiedSharpeRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float,
        annual_risk_free_rate: float,
        confidence_level: float,
        initial_equity: float | None,
        nan_policy: str,
        column: str | None,
    ) -> "ModifiedSharpeRatio":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_risk_free_rate),
            float(confidence_level),
            initial_equity,
            nan_policy,
        )
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ModifiedSharpeRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            confidence_level=confidence_level, initial_equity=None,
            nan_policy=nan_policy, column=column,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ModifiedSharpeRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns, "log_returns", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            confidence_level=confidence_level, initial_equity=None,
            nan_policy=nan_policy, column=column,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ModifiedSharpeRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity, "equity", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            confidence_level=confidence_level, initial_equity=None,
            nan_policy=nan_policy, column=column,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        confidence_level: float = 0.95,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ModifiedSharpeRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl, "pnl", periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            confidence_level=confidence_level, initial_equity=float(initial_equity),
            nan_policy=nan_policy, column=column,
        )

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
