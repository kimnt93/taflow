"""Benchmark-relative active-return dispersion metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import TrackingError as _Native
from ._input import as_paired_metric_series


class TrackingError:
    """Compute sample standard deviation of aligned active returns.

    Active return is primary simple return minus benchmark simple return.
    Sample standard deviation uses one degree of freedom and is multiplied by
    ``sqrt(periods_per_year)`` when ``annualized=True`` (the default). Set
    ``annualized=False`` for per-period tracking error. The independent oracle
    is NumPy ``std(primary - benchmark, ddof=1)``; pandas ``Series.std`` and
    QuantStats' information-ratio denominator provide definition cross-checks.

    At least two usable aligned pairs are required; during this warm-up, empty
    and one-pair states yield ``None``. Missing values are omitted pairwise under the
    default ``nan_policy="omit"`` or rejected by ``"raise"``; infinities and
    mismatched series lengths are rejected. Inputs may be aligned decimal
    simple returns, log returns, positive equity levels, or non-cumulative
    period P&L. P&L conversion requires separate positive initial equity for
    the primary and benchmark series. The first pair of equity levels sets
    baselines and does not increment the length. Mutating lifecycle methods
    are fluent, and Rust owns conversion and O(1)-memory metric arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a paired semantic factory."""
        raise TypeError(
            "use TrackingError.from_returns/from_equity/from_pnl/"
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
    ) -> "TrackingError":
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
    ) -> "TrackingError":
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
    ) -> "TrackingError":
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
    ) -> "TrackingError":
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
    ) -> "TrackingError":
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

    def append(self, value: float, benchmark_value: float) -> "TrackingError":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(self, values: Any, benchmark_values: Any) -> "TrackingError":
        """Append aligned series after validating equal length and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return current tracking error, or ``None`` before two usable pairs."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "TrackingError":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable aligned return pairs processed by Rust."""
        return len(self._state)


__all__ = ["TrackingError"]
