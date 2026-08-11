"""Annualized lower-partial-moment downside deviation."""

from __future__ import annotations

from typing import Any

from .._native.metrics import DownsideDeviation as _Native
from ._input import as_metric_series


class DownsideDeviation:
    """Compute annualized downside deviation below a required return.

    Rust converts ``annual_required_return`` from an annual effective rate to
    an equivalent per-period rate, clips each normalized simple-return excess
    at zero, averages its square over *all* usable observations, and multiplies
    the square root by ``sqrt(periods_per_year)``. This matches Empyrical
    Reloaded 0.5.12 ``downside_risk`` after adapting the oracle's per-period
    target convention. One usable return is sufficient; an empty state yields
    ``None`` during warm-up. Inputs may be decimal simple returns, log returns, positive equity
    levels, or non-cumulative period P&L. The P&L factory requires positive
    initial equity and Rust performs causal capital conversion. The first equity
    level establishes a baseline and does not increment length. ``nan_policy``
    is ``"omit"`` or ``"raise"``; infinities are rejected. Mutating lifecycle
    methods are fluent, and conversion and arithmetic remain in Rust using O(1)
    memory.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use DownsideDeviation.from_returns/from_equity/from_pnl/"
            "from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        annual_required_return: float = 0.0,
        periods_per_year: float = 252.0,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "DownsideDeviation":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_required_return),
            initial_equity,
            nan_policy,
        )
        return state.extend(values)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        annual_required_return: float = 0.0,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "DownsideDeviation":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns,
            "returns",
            annual_required_return=annual_required_return,
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        annual_required_return: float = 0.0,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "DownsideDeviation":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            annual_required_return=annual_required_return,
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        annual_required_return: float = 0.0,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "DownsideDeviation":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity,
            "equity",
            annual_required_return=annual_required_return,
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        annual_required_return: float = 0.0,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "DownsideDeviation":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            annual_required_return=annual_required_return,
            periods_per_year=periods_per_year,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "DownsideDeviation":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "DownsideDeviation":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return annualized downside deviation, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "DownsideDeviation":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["DownsideDeviation"]
