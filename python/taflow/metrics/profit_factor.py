"""Gross-profit to gross-loss quality metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import ProfitFactor as _Native
from ._input import as_metric_series


class ProfitFactor:
    """Compute gross positive observations divided by absolute gross losses.

    Strictly positive values contribute to gross profit, strictly negative
    values contribute to signed gross loss, and exact zeros contribute to
    neither sum. The result matches QuantStats 0.0.81 ``profit_factor`` with
    ``prepare_returns=False`` for defined finite cases; this is the independent
    executable oracle mapping. TAFlow deliberately
    normalizes empty and all-zero input to ``None``. Positive-only input returns
    positive infinity, while loss-only input returns ``0.0``.

    ``from_returns`` consumes decimal simple period returns. ``from_pnl``
    consumes non-cumulative raw period P&L and accepts no initial capital;
    ``from_trades`` consumes realized P&L for closed trades. Values supplied to
    P&L and trade input methods are never converted or annualized. NaNs are omitted
    by default or rejected by ``nan_policy="raise"``; infinities are always
    rejected. Warm-up requires one nonzero usable observation. Mutating lifecycle
    methods are fluent, native bulk work releases the GIL, and Rust owns all
    allocation-free O(1) arithmetic. vectorbt is a documented trade-level
    cross-check but is not installed in the pinned test environment.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "ProfitFactor":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_pnl(
        self, pnl: Any, *, column: str | None = None
    ) -> "ProfitFactor":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(
        self, trades: Any, *, column: str | None = None
    ) -> "ProfitFactor":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

    def append(self, value: float) -> "ProfitFactor":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "ProfitFactor":
        """Append chronological observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return profit factor, including positive infinity when unbounded."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "ProfitFactor":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable observations processed by Rust."""
        return len(self._state)


__all__ = ["ProfitFactor"]
