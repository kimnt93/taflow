"""Profitable-observation frequency metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import WinRate as _Native
from ._input import as_metric_series


class WinRate:
    """Compute strictly positive observations divided by non-zero observations.

    A win is strictly greater than zero, a loss is strictly less than zero,
    and exact zero is a breakeven excluded from the ratio denominator. The
    external oracle is QuantStats 0.0.81 ``win_rate`` with no aggregation and
    ``prepare_returns=False``. TAFlow normalizes QuantStats' arbitrary zero for
    a sample with no non-zero observations to ``None`` because the ratio is
    mathematically undefined. Warm-up ends with the first win or loss; zero is
    a valid result after a loss-only sample. ``len(metric)`` counts every usable
    observation, including breakevens, while omitted NaNs do not advance it.

    ``from_returns`` consumes chronological decimal simple period returns.
    ``from_pnl`` consumes raw, non-cumulative period P&L and deliberately does
    not accept initial equity. ``from_trades`` consumes realized P&L for each
    closed trade. These domains preserve sign classification without conversion
    and are not annualized. Equity and log-return factories are intentionally
    absent. NaNs are omitted by default or rejected with
    ``nan_policy="raise"``; infinities are always rejected, and return-domain
    values below -100% are invalid. ``append``, ``extend``, and ``reset`` are
    fluent. Bulk work releases the GIL; all counting and arithmetic live in Rust.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use WinRate.from_returns/from_pnl/from_trades")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "WinRate":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, nan_policy=nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "WinRate":
        """Construct from chronological decimal simple period returns."""
        return cls._create(
            returns, "returns", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "WinRate":
        """Construct from raw non-cumulative period P&L without conversion."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)

    @classmethod
    def from_trades(
        cls,
        trades: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "WinRate":
        """Construct from realized P&L observations for closed trades."""
        return cls._create(trades, "trades", nan_policy=nan_policy, column=column)

    def append(self, value: float) -> "WinRate":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "WinRate":
        """Append a series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the current win rate, or ``None`` without a win or loss."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "WinRate":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return usable observations including zero-valued breakevens."""
        return len(self._state)


__all__ = ["WinRate"]
