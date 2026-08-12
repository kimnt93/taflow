"""Average-win to average-loss payoff ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import PayoffRatio as _Native
from ._input import as_metric_series


class PayoffRatio:
    """Compute average win divided by the absolute average loss.

    Wins are strictly positive observations, losses are strictly negative, and
    exact zero is breakeven. Warm-up ends only after at least one win and one
    loss have been processed. If either side is absent, ``value`` and
    ``compute`` return ``None``. The external oracle and name mapping is
    QuantStats 0.0.81 ``payoff_ratio`` with return preparation disabled; no
    aggregation or annualization is performed.

    Use ``from_returns`` for decimal simple period returns, ``from_pnl`` for raw
    non-cumulative period P&L, and ``from_trades`` for realized P&L from closed
    trades. Raw P&L and trades are consumed without capital conversion and
    accept no initial capital argument. Equity and log-return input methods are not
    exposed because this quality metric preserves the declared observation
    domain. Rust owns O(1)-memory arithmetic. NaNs are omitted by default or
    rejected with ``nan_policy="raise"``; infinities are always rejected.
    Mutating lifecycle methods are fluent and bulk execution releases the GIL.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "PayoffRatio":
        """Append chronological returns observations and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_pnl(
        self, pnl: Any, *, column: str | None = None
    ) -> "PayoffRatio":
        """Append chronological pnl observations and return this metric."""
        self._state.from_pnl(as_metric_series(pnl, column=column))
        return self

    def from_trades(
        self, trades: Any, *, column: str | None = None
    ) -> "PayoffRatio":
        """Append chronological trades observations and return this metric."""
        self._state.from_trades(as_metric_series(trades, column=column))
        return self

    def append(self, value: float) -> "PayoffRatio":
        """Append one observation in the selected domain and return self."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "PayoffRatio":
        """Append chronological observations in the selected domain and return self."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the payoff ratio, or ``None`` until both sides exist."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed observations."""
        return self._state.compute()

    def reset(self) -> "PayoffRatio":
        """Clear observations, preserve the domain, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable observations processed by Rust."""
        return len(self._state)


__all__ = ["PayoffRatio"]
