"""Closed-trade System Quality Number metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import SystemQualityNumber as _Native
from ._input import as_metric_series


class SystemQualityNumber:
    """Compute System Quality Number from realized closed-trade P&L.

    The formula is ``sqrt(n) * mean(trade_pnl) / sample_std(trade_pnl)``
    with ``ddof=1``. Warm-up requires two usable trades, and a zero sample
    standard deviation makes the result undefined, so ``value`` and
    ``compute`` return ``None`` in either case. A defined result retains the
    sign of mean trade P&L.

    The primary oracle mapping is vectorbt 0.28.1 ``Trades.sqn``. Its source
    tarball is pinned by SHA-256
    ``aceeb4767a1bd5be18329bc85779b2fc744b1edc4513ad19c4dbc3fc7d83d301``;
    vectorbt is not installed in the test environment, so NumPy 2.4.6
    ``mean`` and ``std(ddof=1)`` provide the executable independent numerical
    cross-check without constructing synthetic portfolio records.

    Only ``from_trades`` is available, and each input is realized P&L for one
    chronological closed trade. Period returns, equity, and period P&L are not
    accepted or annualized. Rust owns numerically stable Welford moments and
    O(1)-memory state. NaNs are omitted by default or rejected with
    ``nan_policy="raise"``; infinities are always rejected. Mutating lifecycle
    methods are fluent and bulk execution releases the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use ``from_trades``."""
        raise TypeError("use SystemQualityNumber.from_trades")

    @classmethod
    def from_trades(
        cls,
        trades: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "SystemQualityNumber":
        """Construct from realized P&L for chronological closed trades."""
        state = cls.__new__(cls)
        state._state = _Native("trades", nan_policy)
        return state.extend(trades, column=column)

    def append(self, trade_pnl: float) -> "SystemQualityNumber":
        """Append one realized closed-trade P&L and return this metric."""
        self._state.append(float(trade_pnl))
        return self

    def extend(
        self, trades: Any, *, column: str | None = None
    ) -> "SystemQualityNumber":
        """Append chronological closed-trade P&L and return this metric."""
        self._state.extend(as_metric_series(trades, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return SQN, or ``None`` for insufficient or constant trade P&L."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed trades."""
        return self._state.compute()

    def reset(self) -> "SystemQualityNumber":
        """Clear trades, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable closed trades processed by Rust."""
        return len(self._state)


__all__ = ["SystemQualityNumber"]
