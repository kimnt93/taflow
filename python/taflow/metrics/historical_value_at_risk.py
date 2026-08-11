"""Exact historical value-at-risk metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import HistoricalValueAtRisk as _Native
from ._input import as_metric_series


class HistoricalValueAtRisk:
    """Compute the signed lower-tail linear quantile of simple returns.

    ``cutoff`` is a probability strictly between zero and one and defaults to
    ``0.05``. The result follows NumPy's linear percentile convention and the
    independent Empyrical Reloaded 0.5.12 ``value_at_risk`` oracle; it is a
    signed return, so a typical loss-tail result is negative. The warm-up is
    one usable return: an empty state returns ``None`` and the first accepted
    observation is immediately defined. Inputs may be decimal
    simple returns, log returns, positive equity levels, or non-cumulative
    period P&L. The P&L factory requires positive initial equity and Rust
    performs causal capital conversion. The first equity level establishes a
    baseline and does not increment metric length. ``nan_policy`` is ``"omit"``
    or ``"raise"``; infinities are rejected. Mutating lifecycle methods are
    fluent. Rust retains O(n) observations for exact order statistics and
    lazily refreshes a sorted cache; Python performs no metric arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use HistoricalValueAtRisk.from_returns/from_equity/from_pnl/"
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
    ) -> "HistoricalValueAtRisk":
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
    ) -> "HistoricalValueAtRisk":
        """Construct from chronological decimal simple returns."""
        return cls._create(
            returns, "returns", cutoff=cutoff, nan_policy=nan_policy
        )

    @classmethod
    def from_log_returns(
        cls,
        log_returns: Any,
        *,
        cutoff: float = 0.05,
        nan_policy: str = "omit",
    ) -> "HistoricalValueAtRisk":
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
    ) -> "HistoricalValueAtRisk":
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
    ) -> "HistoricalValueAtRisk":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            cutoff=cutoff,
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "HistoricalValueAtRisk":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "HistoricalValueAtRisk":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the current signed quantile, or ``None`` for an empty state."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current exact scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "HistoricalValueAtRisk":
        """Clear retained observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns retained by Rust."""
        return len(self._state)


__all__ = ["HistoricalValueAtRisk"]
