"""Signed gross-loss metric for raw P&L observations."""

from __future__ import annotations

from typing import Any

from .._native.metrics import GrossLoss as _Native
from ._input import as_metric_series


class GrossLoss:
    """Compute the signed sum of strictly negative raw P&L values.

    Negative observations are added without changing their sign; positive and
    exact-zero observations contribute zero. Warm-up requires one usable P&L
    observation. Empty input returns ``None``, while a non-empty history with
    no loss validly returns ``0.0``. The independent oracle is NumPy's strict
    negative filtered sum. QuantStats 0.0.81 ``profit_factor`` provides an
    executable cross-check through the absolute value of the same denominator
    when called with ``prepare_returns=False``.

    ``from_pnl`` consumes non-cumulative raw period P&L and ``from_trades``
    consumes realized P&L from closed trades. Both preserve monetary values
    exactly and accept no initial-equity argument. Return, equity, and
    log-return factories are deliberately unavailable because Gross Loss is an
    absolute P&L statistic. Rust owns O(1)-memory arithmetic. NaNs are omitted
    by default or rejected with ``nan_policy="raise"``; infinities are always
    rejected. Mutating lifecycle methods are fluent and bulk work releases the
    GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use GrossLoss.from_pnl/from_trades")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str,
        column: str | None,
    ) -> "GrossLoss":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GrossLoss":
        """Construct from raw non-cumulative period P&L without conversion."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)

    @classmethod
    def from_trades(
        cls,
        trades: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GrossLoss":
        """Construct from realized P&L for chronological closed trades."""
        return cls._create(trades, "trades", nan_policy=nan_policy, column=column)

    def append(self, value: float) -> "GrossLoss":
        """Append one raw P&L observation and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "GrossLoss":
        """Append chronological raw P&L observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return signed gross loss, or ``None`` while empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed P&L."""
        return self._state.compute()

    def reset(self) -> "GrossLoss":
        """Clear observations, preserve the P&L domain, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable P&L observations processed by Rust."""
        return len(self._state)


__all__ = ["GrossLoss"]
