"""Mean strictly positive return or profit metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import AverageWin as _Native
from ._input import as_metric_series


class AverageWin:
    """Compute the arithmetic mean of strictly positive observations.

    Zero is breakeven and negative observations are losses; neither enters the
    numerator or win count, although every usable observation contributes to
    ``len(metric)``. Warm-up ends with the first strictly positive value.
    Empty histories and histories without a win return ``None``. The external
    oracle and name mapping is QuantStats 0.0.81 ``avg_win`` with preparation
    disabled and no aggregation, which directly evaluates the supplied input
    resolution.

    Use ``from_returns`` for decimal simple period returns, ``from_pnl`` for raw
    non-cumulative period P&L, and ``from_trades`` for realized P&L from closed
    trades. The latter two domains are consumed without capital conversion and
    accept no ``initial_capital`` argument. This metric deliberately exposes no
    equity or log-return input method because their implicit conversion would blur
    the declared observation meaning. Rust owns O(1)-memory arithmetic and
    missing-value handling. NaNs are omitted by default or rejected with
    ``nan_policy="raise"``; infinities are always rejected. Mutating lifecycle
    methods are fluent and bulk execution releases the GIL.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "AverageWin":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_pnl(
        self, pnl: Any, *, column: str | None = None
    ) -> "AverageWin":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(
        self, trades: Any, *, column: str | None = None
    ) -> "AverageWin":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

    def append(self, value: float) -> "AverageWin":
        """Append one observation in the selected domain and return self."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "AverageWin":
        """Append chronological observations in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the mean strictly positive observation, or ``None`` without wins."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed observations."""
        return self._state.compute()

    def reset(self) -> "AverageWin":
        """Clear observations, preserve the domain, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable observations processed by Rust."""
        return len(self._state)


__all__ = ["AverageWin"]
