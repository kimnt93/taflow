"""Expected raw period or closed-trade profit metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import Expectancy as _Native
from ._input import as_metric_series


class Expectancy:
    """Compute expected P&L per period or closed trade.

    The formula is ``P(win) * average_win + P(loss) * average_loss``. Wins are
    strictly positive, losses strictly negative, and both probabilities divide
    by every usable observation. Breakevens therefore contribute zero while
    remaining in the denominator. Algebraically this equals signed net P&L
    divided by observation count. The independent oracle uses QuantStats 0.0.81
    ``avg_win`` and ``avg_loss`` with preparation disabled, combined with
    explicit all-observation probabilities. Empty/all-missing warm-up returns
    ``None``; an all-breakeven sample returns ``0.0``.

    ``from_pnl`` consumes raw non-cumulative period P&L and deliberately accepts
    no initial equity. ``from_trades`` consumes realized P&L observations for
    closed trades. Returns, equity, and log-return factories are intentionally
    absent because this metric's output is denominated in the supplied monetary
    unit. Values are neither converted nor annualized. NaNs are omitted by
    default or rejected with ``nan_policy="raise"``; infinities are always
    rejected. ``append``, ``extend``, and ``reset`` are fluent. Native bulk work
    releases the GIL, and Rust owns allocation-free O(1) arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use Expectancy.from_pnl/from_trades")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "Expectancy":
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
    ) -> "Expectancy":
        """Construct from raw non-cumulative period P&L without conversion."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)

    @classmethod
    def from_trades(
        cls,
        trade_pnl: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "Expectancy":
        """Construct from realized P&L observations for closed trades."""
        return cls._create(
            trade_pnl, "trades", nan_policy=nan_policy, column=column
        )

    def append(self, value: float) -> "Expectancy":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "Expectancy":
        """Append chronological P&L observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return expected P&L per observation, or ``None`` while empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "Expectancy":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable P&L observations processed by Rust."""
        return len(self._state)


__all__ = ["Expectancy"]
