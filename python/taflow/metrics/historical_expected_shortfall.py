"""Exact historical expected-shortfall metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import HistoricalExpectedShortfall as _Native
from ._input import as_metric_series


class HistoricalExpectedShortfall:
    """Compute the signed mean of the exact lower tail of simple returns.

    For ``n`` usable returns, the metric averages the lowest
    ``floor((n - 1) * cutoff) + 1`` observations. ``cutoff`` must be strictly
    between zero and one and defaults to ``0.05``. This contract matches the
    independent Empyrical Reloaded 0.5.12 ``conditional_value_at_risk`` oracle.
    The result is signed, so a typical loss-tail result is negative. One usable
    return is sufficient; before that warm-up an empty state returns ``None``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. The P&L factory requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities are rejected.
    ``append``, ``extend``, and ``reset`` are fluent. Rust retains O(n)
    observations and lazily refreshes its exact sorted cache; Python performs
    no conversion or metric arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use HistoricalExpectedShortfall.from_returns/from_equity/from_pnl/"
            "from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        cutoff: float = 0.05,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "HistoricalExpectedShortfall":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, float(cutoff), initial_equity, nan_policy)
        return state.extend(values)

    @classmethod
    def from_returns(
        cls,
        returns: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
    ) -> "HistoricalExpectedShortfall":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", cutoff=cutoff, nan_policy=nan_policy)

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
    ) -> "HistoricalExpectedShortfall":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(
            log_returns, "log_returns", cutoff=cutoff, nan_policy=nan_policy
        )

    @classmethod
    def from_equity(
        cls,
        equity: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
    ) -> "HistoricalExpectedShortfall":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", cutoff=cutoff, nan_policy=nan_policy)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
    ) -> "HistoricalExpectedShortfall":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            cutoff=cutoff,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "HistoricalExpectedShortfall":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "HistoricalExpectedShortfall":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the current signed tail mean, or ``None`` for an empty state."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current exact scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "HistoricalExpectedShortfall":
        """Clear retained observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns retained by Rust."""
        return len(self._state)


__all__ = ["HistoricalExpectedShortfall"]
