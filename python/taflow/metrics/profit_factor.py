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
    consumes non-cumulative raw period P&L and accepts no initial equity;
    ``from_trades`` consumes realized P&L for closed trades. Values supplied to
    P&L and trade factories are never converted or annualized. NaNs are omitted
    by default or rejected by ``nan_policy="raise"``; infinities are always
    rejected. Warm-up requires one nonzero usable observation. Mutating lifecycle
    methods are fluent, native bulk work releases the GIL, and Rust owns all
    allocation-free O(1) arithmetic. vectorbt is a documented trade-level
    cross-check but is not installed in the pinned test environment.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use ProfitFactor.from_returns/from_pnl/from_trades")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProfitFactor":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProfitFactor":
        """Construct from chronological decimal simple returns."""
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
    ) -> "ProfitFactor":
        """Construct from raw non-cumulative period P&L without capital conversion."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)

    @classmethod
    def from_trades(
        cls,
        trade_pnl: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "ProfitFactor":
        """Construct from realized P&L observations for closed trades."""
        return cls._create(
            trade_pnl, "trades", nan_policy=nan_policy, column=column
        )

    def append(self, value: float) -> "ProfitFactor":
        """Append one value in the factory-selected domain and return this metric."""
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
