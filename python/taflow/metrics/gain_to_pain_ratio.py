"""Whole-history gain-to-pain ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import GainToPainRatio as _Native
from ._input import as_metric_series


class GainToPainRatio:
    """Compute net return sum divided by the absolute sum of losses.

    The frozen external oracle is QuantStats 0.0.81
    ``gain_to_pain_ratio``. Despite the metric's common gross-gain wording,
    that implementation uses ``sum(all returns) / abs(sum(negative returns))``;
    TAFlow deliberately follows that executable convention. QuantStats also
    ignores its ``rf`` argument for this metric, so TAFlow exposes no rate
    parameter. Phase 1 performs no implicit date-based resampling: every input
    observation must already represent the caller's intended aggregation
    resolution.

    Warm-up ends as soon as at least one negative return supplies a nonzero
    pain denominator. The result is ``None`` for empty, all-zero, and gain-only
    histories; a loss-only history validly returns ``-1.0``. Inputs may be
    decimal simple returns, log returns, positive equity levels, or
    non-cumulative period P&L with positive initial equity. Rust performs all
    conversions and O(1)-memory arithmetic. NaNs are omitted by default or
    rejected with ``nan_policy="raise"``; infinities are always rejected.
    Mutating lifecycle methods are fluent and bulk work releases the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use GainToPainRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GainToPainRatio":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, initial_equity, nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GainToPainRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GainToPainRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns, "log_returns", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GainToPainRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy, column=column)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GainToPainRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
            column=column,
        )

    def append(self, value: float) -> "GainToPainRatio":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "GainToPainRatio":
        """Append a chronological input-resolution series and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return net gain divided by pain, or ``None`` without a loss."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "GainToPainRatio":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["GainToPainRatio"]
