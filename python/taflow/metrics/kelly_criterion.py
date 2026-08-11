"""Historical binary Kelly-fraction metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import KellyCriterion as _Native
from ._input import as_metric_series


class KellyCriterion:
    """Estimate the historical binary Kelly fraction from observed outcomes.

    For strictly positive wins and strictly negative losses, let ``p`` be wins
    divided by decisive non-zero observations, ``q = 1 - p``, and ``b`` be
    average win divided by absolute average loss. The result is
    ``((b * p) - q) / b``. Exact-zero breakevens are excluded from ``p`` and
    ``q`` but included in ``len(metric)``. Warm-up requires at least one win and
    one loss; empty, all-breakeven, win-only, loss-only, zero-payoff, and
    non-finite cases return ``None``. A valid result may be negative.

    The external oracle and name mapping is QuantStats 0.0.81
    ``kelly_criterion``. TAFlow freezes its binary historical formula on the
    supplied observation resolution without adopting QuantStats' heuristic
    preparation. This statistic describes historical outcomes; it does not
    execute or recommend an order-sizing action.

    ``from_returns`` consumes chronological decimal simple period returns.
    ``from_trades`` consumes realized P&L for chronological closed trades with
    no capital conversion. Raw period P&L is intentionally unsupported by the
    catalog contract. NaNs are omitted by default or rejected with
    ``nan_policy="raise"``; infinities are always rejected, and return-domain
    values below -100% are invalid. ``append``, ``extend``, and ``reset`` are
    fluent. Rust owns O(1)-memory arithmetic, and native bulk execution releases
    the GIL.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use KellyCriterion.from_returns/from_trades")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        nan_policy: str,
        column: str | None,
    ) -> "KellyCriterion":
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
    ) -> "KellyCriterion":
        """Construct from chronological decimal simple period returns."""
        return cls._create(
            returns, "returns", nan_policy=nan_policy, column=column
        )

    @classmethod
    def from_trades(
        cls,
        trades: Any,
        *,
        nan_policy: str = "omit",
        column: str | None = None,
    ) -> "KellyCriterion":
        """Construct from realized P&L for chronological closed trades."""
        return cls._create(trades, "trades", nan_policy=nan_policy, column=column)

    def append(self, value: float) -> "KellyCriterion":
        """Append one observation and return this metric."""
        self._state.append(float(value))
        return self

    def extend(
        self, values: Any, *, column: str | None = None
    ) -> "KellyCriterion":
        """Append chronological observations and return this metric."""
        self._state.extend(as_metric_series(values, column=column))
        return self

    @property
    def value(self) -> float | None:
        """Return the Kelly fraction, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed observations."""
        return self._state.compute()

    def reset(self) -> "KellyCriterion":
        """Clear observations, preserve the domain, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return usable observations including zero-valued breakevens."""
        return len(self._state)


__all__ = ["KellyCriterion"]
