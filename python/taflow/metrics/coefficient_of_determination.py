"""Benchmark-relative coefficient of determination metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import CoefficientOfDetermination as _Native
from ._input import as_paired_metric_series


class CoefficientOfDetermination:
    """Compute squared Pearson correlation with a benchmark.

    The metric consumes aligned decimal simple returns and squares their
    Pearson correlation coefficient. The independent oracle is QuantStats 0.0.81
    ``r_squared`` and returns a value on the zero-to-one scale, subject only to
    floating-point rounding. Warm-up requires at least two usable aligned
    pairs. Empty and one-pair states, or either series having zero variance,
    yield ``None``.

    Missing values are omitted pairwise under ``nan_policy="omit"`` or
    rejected under ``"raise"``. Infinities and mismatched series lengths are
    rejected before native mutation. Inputs may be aligned simple returns,
    log returns, positive equity levels, or non-cumulative period P&L. Period
    P&L requires separate positive initial capital for each series. The first
    equity-level pair establishes conversion baselines and does not increment
    the length. Mutating lifecycle methods are fluent; Rust owns conversions
    and allocation-free O(1) paired moments.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a paired semantic factory."""
        raise TypeError(
            "use CoefficientOfDetermination."
            "from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        benchmark_values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        benchmark_initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "CoefficientOfDetermination":
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
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
        nan_policy: str = "omit",
    ) -> "CoefficientOfDetermination":
        """Construct from aligned chronological decimal simple returns."""
        return cls._create(
            returns,
            benchmark_returns,
            "returns",
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        benchmark_log_returns: Any,
        *,
        nan_policy: str = "omit",
    ) -> "CoefficientOfDetermination":
        """Construct from aligned chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            benchmark_log_returns,
            "log_returns",
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        benchmark_equity: Any,
        *,
        nan_policy: str = "omit",
    ) -> "CoefficientOfDetermination":
        """Construct from aligned positive equity or adjusted-price levels."""
        return cls._create(
            equity,
            benchmark_equity,
            "equity",
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
        nan_policy: str = "omit",
    ) -> "CoefficientOfDetermination":
        """Construct from aligned period P&L and separate initial capitals."""
        return cls._create(
            pnl,
            benchmark_pnl,
            "pnl",
            initial_equity=float(initial_equity),
            benchmark_initial_equity=float(benchmark_initial_equity),
            nan_policy=nan_policy,
        )

    def append(
        self, value: float, benchmark_value: float
    ) -> "CoefficientOfDetermination":
        """Append one aligned pair in the selected domain and return this metric."""
        self._state.append(float(value), float(benchmark_value))
        return self

    def extend(
        self, values: Any, benchmark_values: Any
    ) -> "CoefficientOfDetermination":
        """Append aligned series after validating length and return this metric."""
        primary, benchmark = as_paired_metric_series(values, benchmark_values)
        self._state.extend(primary, benchmark)
        return self

    @property
    def value(self) -> float | None:
        """Return current squared correlation, or ``None`` until defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "CoefficientOfDetermination":
        """Clear observations, preserve input settings, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the usable aligned-return count delegated to native state."""
        return len(self._state)


__all__ = ["CoefficientOfDetermination"]
