"""Absolute gross-positive P&L metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import GrossProfit as _Native
from ._input import as_metric_series


class GrossProfit:
    """Sum strictly positive raw period or closed-trade P&L observations.

    Positive values enter the sum; losses and exact-zero breakevens contribute
    zero. Empty input returns ``None`` during warm-up, while a non-empty history
    with no profit returns the valid value ``0.0``. ``len(metric)`` counts every
    usable P&L observation. The primary external oracle is NumPy's sum over the
    strictly positive values. QuantStats 0.0.81 ``profit_factor`` with
    ``prepare_returns=False`` cross-checks the same numerator on a series whose
    absolute gross loss is one.

    ``from_pnl`` consumes raw, non-cumulative period P&L and deliberately does
    not accept initial equity. ``from_trades`` consumes realized P&L for each
    closed trade. Returns, log returns, and equity are intentionally unsupported
    because this metric is an absolute currency-domain statistic, not a return
    measure. NaNs are omitted by default or rejected with
    ``nan_policy="raise"``; infinities are always rejected. ``append``,
    ``extend``, and ``reset`` mutate and fluently return this metric. Rust owns
    O(1)-memory arithmetic, and native bulk execution releases the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use GrossProfit.from_pnl/from_trades")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str,
        column: str | None,
    ) -> "GrossProfit":
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
    ) -> "GrossProfit":
        """Construct from raw non-cumulative period P&L without conversion."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)

    @classmethod
    def from_trades(
        cls,
        trades: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "GrossProfit":
        """Construct from realized P&L for chronological closed trades."""
        return cls._create(trades, "trades", nan_policy=nan_policy, column=column)

    def append(self, value: float) -> "GrossProfit":
        """Append one P&L observation and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "GrossProfit":
        """Append chronological P&L observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return gross profit, or ``None`` before any usable observation."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed observations."""
        return self._state.compute()

    def reset(self) -> "GrossProfit":
        """Clear observations, preserve the P&L domain, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable P&L observations processed by Rust."""
        return len(self._state)


__all__ = ["GrossProfit"]
