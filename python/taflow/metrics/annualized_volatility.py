"""Annualized return-volatility metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import AnnualizedVolatility as _Native
from ._input import as_metric_series


class AnnualizedVolatility:
    """Compute annualized sample standard deviation of simple returns.

    The result is the sample standard deviation (one degree of freedom) times
    ``sqrt(periods_per_year)``. The independent oracle is Empyrical Reloaded 0.5.12
    ``annual_volatility`` with its default Levy alpha of two. At least two
    usable returns are required; this two-observation warm-up means an empty or one-return state yields ``None``,
    while a constant sample yields zero. Inputs may be decimal simple returns,
    log returns, positive equity levels, or non-cumulative period P&L. The P&L
    factory requires positive initial equity and Rust performs causal capital
    conversion. The first equity level establishes a baseline and does not
    increment the metric length. ``nan_policy`` is ``"omit"`` or ``"raise"``;
    infinities are rejected. Mutating lifecycle methods are fluent, and all
    conversion, state, and metric arithmetic live in Rust using O(1) memory.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use AnnualizedVolatility.from_returns/from_equity/from_pnl/"
            "from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float = 252.0,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "AnnualizedVolatility":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode, float(periods_per_year), initial_equity, nan_policy
        )
        return state.extend(values)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "AnnualizedVolatility":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns,
            "returns",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "AnnualizedVolatility":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "AnnualizedVolatility":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity,
            "equity",
            periods_per_year=periods_per_year,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        periods_per_year: float = 252.0,
        nan_policy: str = "omit",
    ) -> "AnnualizedVolatility":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            periods_per_year=periods_per_year,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "AnnualizedVolatility":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "AnnualizedVolatility":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return annualized sample volatility, or ``None`` before two returns."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "AnnualizedVolatility":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["AnnualizedVolatility"]
