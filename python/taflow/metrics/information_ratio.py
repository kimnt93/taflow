"""Benchmark-relative active-return information ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import InformationRatio as _Native
from ._input import as_paired_metric_series


class InformationRatio:
    """Compute mean aligned active return divided by tracking error.

    Active return is primary simple return minus benchmark simple return, and
    tracking error is its sample standard deviation with one degree of freedom.
    The result is multiplied by ``sqrt(periods_per_year)`` when
    ``annualized=True`` (the default); set ``annualized=False`` for the
    per-period ratio. The independent oracles are Empyrical Reloaded 0.5.12
    ``excess_sharpe`` and QuantStats 0.0.81 ``information_ratio`` for the
    unannualized definition; TAFlow's annualized variant applies the explicitly
    documented scale.

    At least two usable aligned pairs and nonzero tracking error are required;
    otherwise warm-up or a zero denominator yields ``None``. Missing values are
    omitted pairwise under the default ``nan_policy="omit"`` or rejected by
    ``"raise"``. Infinities and mismatched series lengths are rejected before
    state mutation. Inputs
    may be aligned decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. P&L conversion requires separate positive
    initial equity for primary and benchmark streams. The first equity pair
    establishes baselines and does not increment length. Mutating lifecycle
    methods are fluent; Rust owns conversion and O(1)-memory arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a paired semantic factory."""
        raise TypeError(
            "use InformationRatio.from_returns/from_equity/from_pnl/"
            "from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        benchmark_values: Any,
        input_mode: str,
        *,
        periods_per_year: float = 252.0,
        annualized: bool = True,
        initial_equity: float | None = None,
        benchmark_initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "InformationRatio":
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            bool(annualized),
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
        annualized: bool = True,
        nan_policy: str = "omit",
    ) -> "InformationRatio":
        """Construct from aligned chronological decimal simple returns."""
        return cls._create(
            returns,
            benchmark_returns,
            "returns",
            periods_per_year=periods_per_year,
            annualized=annualized,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        benchmark_log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annualized: bool = True,
        nan_policy: str = "omit",
    ) -> "InformationRatio":
        """Construct from aligned chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            benchmark_log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            annualized=annualized,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        benchmark_equity: Any,
        *,
        periods_per_year: float = 252.0,
        annualized: bool = True,
        nan_policy: str = "omit",
    ) -> "InformationRatio":
        """Construct from aligned positive equity or adjusted-price levels."""
        return cls._create(
            equity,
            benchmark_equity,
            "equity",
            periods_per_year=periods_per_year,
            annualized=annualized,
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
        annualized: bool = True,
        nan_policy: str = "omit",
    ) -> "InformationRatio":
        """Construct from aligned period P&L and separate initial capitals."""
        return cls._create(
            pnl,
            benchmark_pnl,
            "pnl",
            periods_per_year=periods_per_year,
            annualized=annualized,
            initial_equity=float(initial_equity),
            benchmark_initial_equity=float(benchmark_initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float, benchmark_value: float) -> "InformationRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "InformationRatio":
        """Append aligned series after equal-length validation and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the ratio, or ``None`` until its denominator is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "InformationRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["InformationRatio"]
