"""Longest consecutive winning-observation metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import LongestWinningStreak as _Native
from ._input import as_metric_series


class LongestWinningStreak:
    """Compute the longest consecutive run of strictly positive observations.

    A win is strictly greater than zero. Both exact zero and a negative value
    terminate the current streak. The independent oracle is QuantStats 0.0.81
    ``consecutive_wins`` with no aggregation and ``prepare_returns=False``.
    TAFlow returns ``None`` during empty/all-missing warm-up and ``0.0`` for a
    non-empty sample with no win. With ``nan_policy="omit"``, missing values are
    removed and therefore do not break a run; ``"raise"`` rejects them.

    ``from_returns`` consumes decimal simple period returns. ``from_pnl``
    consumes raw, non-cumulative period P&L without initial-capital conversion,
    and ``from_trades`` consumes realized P&L for closed trades. No domain is
    annualized. Infinities are rejected, while return-domain observations below
    -100% are invalid. ``append``, ``extend``, and ``reset`` are fluent. Native
    bulk work releases the GIL, and Rust owns bounded allocation-free O(1)
    streaming state.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use LongestWinningStreak.from_returns/from_pnl/from_trades"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "LongestWinningStreak":
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
    ) -> "LongestWinningStreak":
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
    ) -> "LongestWinningStreak":
        """Construct from raw non-cumulative period P&L without conversion."""
        return cls._create(pnl, "pnl", nan_policy=nan_policy, column=column)

    @classmethod
    def from_trades(
        cls,
        trade_pnl: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "LongestWinningStreak":
        """Construct from realized P&L observations for closed trades."""
        return cls._create(
            trade_pnl, "trades", nan_policy=nan_policy, column=column
        )

    def append(self, value: float) -> "LongestWinningStreak":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "LongestWinningStreak":
        """Append chronological observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the longest winning run, or ``None`` while empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "LongestWinningStreak":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable observations processed by Rust."""
        return len(self._state)


__all__ = ["LongestWinningStreak"]
