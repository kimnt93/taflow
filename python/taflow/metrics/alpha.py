"""Annualized benchmark-relative regression alpha metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import Alpha as _Native
from ._input import as_paired_metric_series


class Alpha:
    """Compute the annualized intercept of a single-factor return regression.

    For pairwise-aligned decimal simple returns, beta is primary/benchmark
    covariance divided by benchmark variance. The per-period intercept is
    ``mean(primary - risk_free) - beta * mean(benchmark - risk_free)`` and is
    annualized with Empyrical's compounding convention,
    ``(1 + intercept) ** periods_per_year - 1``. TAFlow accepts an annual
    effective risk-free rate and converts it in Rust with
    ``expm1(log1p(rate) / periods_per_year)`` before matching the independent
    Empyrical Reloaded 0.5.12 ``alpha_aligned`` oracle.

    Warm-up requires at least two usable aligned pairs. A benchmark population
    variance below Empyrical's ``1e-30`` threshold, or a non-finite compounded
    result, yields ``None``. Missing values are omitted pairwise with the
    default ``nan_policy="omit"`` or rejected by ``"raise"``; infinities and
    unequal lengths are rejected before native mutation. Inputs may be simple
    returns, log returns, positive equity levels, or non-cumulative period P&L.
    Period P&L requires separate positive initial capital for each stream. The
    first equity pair establishes conversion baselines and does not increment
    length. Mutating lifecycle methods are fluent; Rust owns input conversion
    and allocation-free O(1) arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a paired semantic factory."""
        raise TypeError("use Alpha.from_returns/from_equity/from_pnl/from_log_returns")

    @classmethod
    def _create(
        cls,
        values: Any,
        benchmark_values: Any,
        input_mode: str,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        initial_equity: float | None = None,
        benchmark_initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "Alpha":
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_risk_free_rate),
            initial_equity,
            benchmark_initial_equity,
            nan_policy,
        )
        state._state.extend(primary, benchmark)
        return state

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        benchmark_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "Alpha":
        """Construct from aligned chronological decimal simple returns."""
        return cls._create(
            returns,
            benchmark_returns,
            "returns",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        benchmark_log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "Alpha":
        """Construct from aligned chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            benchmark_log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        benchmark_equity: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "Alpha":
        """Construct from aligned positive equity or adjusted-price levels."""
        return cls._create(
            equity,
            benchmark_equity,
            "equity",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        benchmark_pnl: Any,
        *,
        initial_equity: float,
        benchmark_initial_equity: float,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "Alpha":
        """Construct from aligned period P&L and separate initial capitals."""
        return cls._create(
            pnl,
            benchmark_pnl,
            "pnl",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            initial_equity=float(initial_equity),
            benchmark_initial_equity=float(benchmark_initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float, benchmark_value: float) -> "Alpha":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "Alpha":
        """Append equal-length aligned series and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return annualized alpha, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "Alpha":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["Alpha"]
