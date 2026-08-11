"""Benchmark-relative Treynor ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import TreynorRatio as _Native
from ._input import as_paired_metric_series


class TreynorRatio:
    """Compute geometrically annualized excess return divided by market beta.

    TAFlow follows PerformanceAnalytics 2.1.0 ``TreynorRatio``: subtract the
    same per-period risk-free return from the aligned portfolio and benchmark
    returns, compute CAPM beta, geometrically annualize the portfolio's
    resulting excess-return observations, and divide that annualized return by
    beta. The source convention is pinned to the CRAN 2.1.0 tarball with
    SHA-256 ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.
    The executable independent oracle cross-check is QuantStats 0.0.81
    ``treynor_ratio`` on its mathematically equivalent zero-risk-free subset.
    TAFlow accepts an annual effective risk-free rate and converts it in Rust
    with ``expm1(log1p(rate) / periods_per_year)``.

    Warm-up requires two usable aligned pairs. Zero benchmark variance, zero
    beta, or a non-finite/non-real annualized excess return yields ``None``.
    Missing values are omitted pairwise under ``nan_policy="omit"`` or rejected
    by ``"raise"``; infinities and unequal lengths are rejected before native
    mutation. Inputs may be simple returns, log returns, positive equity
    levels, or non-cumulative period P&L. P&L requires separate positive initial
    capital for each stream. The first equity pair establishes conversion
    baselines and does not increment length. Mutating lifecycle methods are
    fluent; Rust owns conversion and allocation-free O(1) metric arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a paired semantic factory."""
        raise TypeError(
            "use TreynorRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

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
    ) -> "TreynorRatio":
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
    ) -> "TreynorRatio":
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
    ) -> "TreynorRatio":
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
    ) -> "TreynorRatio":
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
    ) -> "TreynorRatio":
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

    def append(self, value: float, benchmark_value: float) -> "TreynorRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "TreynorRatio":
        """Append equal-length aligned series and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the current Treynor ratio, or ``None`` until defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "TreynorRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["TreynorRatio"]
