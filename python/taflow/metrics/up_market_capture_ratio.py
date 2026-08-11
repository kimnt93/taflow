"""Up-market benchmark capture ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import UpMarketCaptureRatio as _Native
from ._input import as_paired_metric_series


class UpMarketCaptureRatio:
    """Compute capture using only periods with positive benchmark return.

    Aligned pairs are first converted to simple returns and missing values are
    omitted pairwise. Pairs whose normalized benchmark return is not strictly
    positive are then excluded. For the remaining observations, the result is
    primary CAGR divided by benchmark CAGR, where each CAGR is
    ``product(1 + return) ** (periods_per_year / eligible_count) - 1``.
    The independent oracle is Empyrical Reloaded 0.5.12 ``up_capture``, with
    the explicit default ``periods_per_year=252.0`` daily convention.

    One eligible observation is sufficient. Warm-up returns ``None`` for empty
    state or while no positive benchmark period has appeared; a zero benchmark
    CAGR is also ``None``. Under
    ``nan_policy="omit"`` a NaN removes its entire aligned pair; ``"raise"``
    rejects it. Infinities and mismatched lengths are rejected before metric
    mutation. Inputs may be aligned simple returns, log returns, positive equity
    levels, or non-cumulative period P&L. Period P&L requires separate positive
    initial capital for both streams. The first equity pair establishes the
    baselines and does not count as an observation. ``len(metric)`` counts all
    usable normalized aligned pairs; the up-market filter affects only the
    formula state. Mutating lifecycle methods are fluent; Rust owns conversion,
    filtering, compounding, and O(1)-memory state.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a paired semantic factory."""
        raise TypeError(
            "use UpMarketCaptureRatio.from_returns/from_equity/from_pnl/"
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
        initial_equity: float | None = None,
        benchmark_initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "UpMarketCaptureRatio":
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
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
        nan_policy: str = "omit",
    ) -> "UpMarketCaptureRatio":
        """Construct from aligned chronological decimal simple returns."""
        return cls._create(
            returns,
            benchmark_returns,
            "returns",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        benchmark_log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "UpMarketCaptureRatio":
        """Construct from aligned chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            benchmark_log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        benchmark_equity: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "UpMarketCaptureRatio":
        """Construct from aligned positive equity or adjusted-price levels."""
        return cls._create(
            equity,
            benchmark_equity,
            "equity",
            periods_per_year=periods_per_year,
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
        nan_policy: str = "omit",
    ) -> "UpMarketCaptureRatio":
        """Construct from aligned period P&L and separate initial capitals."""
        return cls._create(
            pnl,
            benchmark_pnl,
            "pnl",
            periods_per_year=periods_per_year,
            initial_equity=float(initial_equity),
            benchmark_initial_equity=float(benchmark_initial_equity),
            nan_policy=nan_policy,
        )

    def append(
        self, value: float, benchmark_value: float
    ) -> "UpMarketCaptureRatio":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(
        self, values: Any, benchmark_values: Any
    ) -> "UpMarketCaptureRatio":
        """Append equal-length aligned series and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return the ratio, or ``None`` until an eligible result is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "UpMarketCaptureRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized aligned pairs."""
        return len(self._state)


__all__ = ["UpMarketCaptureRatio"]
