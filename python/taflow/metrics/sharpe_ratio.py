"""Annualized excess-return Sharpe ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import SharpeRatio as _Native
from ._input import as_metric_series


class SharpeRatio:
    """Compute annualized mean excess return divided by sample deviation.

    The annual effective risk-free rate is converted in Rust to an effective
    per-period rate with ``expm1(log1p(rate) / periods_per_year)``. The ratio is
    the arithmetic mean of per-period excess simple returns divided by their
    sample standard deviation, multiplied by ``sqrt(periods_per_year)``. This
    matches the independent oracle Empyrical Reloaded 0.5.12
    ``sharpe_ratio`` after adapting its per-period ``risk_free`` argument.
    Warm-up requires at least two usable returns; zero deviation also makes the
    result undefined. Both cases return ``None``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. The P&L factory requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a baseline and does not increment the metric length.
    ``nan_policy`` accepts ``"omit"`` or ``"raise"``; infinities are rejected.
    Mutating lifecycle methods are fluent. Conversion and metric arithmetic
    live in Rust and use O(1) memory.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use SharpeRatio.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "SharpeRatio":
        state = cls.__new__(cls)
        state._state = _Native(
            input_mode,
            float(periods_per_year),
            float(annual_risk_free_rate),
            initial_equity,
            nan_policy,
        )
        return state.extend(values)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "SharpeRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns,
            "returns",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "SharpeRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns,
            "log_returns",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "SharpeRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(
            equity,
            "equity",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            nan_policy=nan_policy,
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        periods_per_year: float = 252.0,
        annual_risk_free_rate: float = 0.0,
        nan_policy: str = "omit",
    ) -> "SharpeRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            periods_per_year=periods_per_year,
            annual_risk_free_rate=annual_risk_free_rate,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "SharpeRatio":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "SharpeRatio":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the current ratio, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "SharpeRatio":
        """Clear observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["SharpeRatio"]
