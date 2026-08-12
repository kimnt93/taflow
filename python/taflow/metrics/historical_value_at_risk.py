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
    period P&L. The P&L input method requires positive initial capital and Rust
    performs causal capital conversion. The first equity level establishes a
    baseline and does not increment metric length. ``nan_policy`` is ``"omit"``
    or ``"raise"``; infinities are rejected. Mutating lifecycle methods are
    fluent. Rust retains O(n) observations for exact order statistics and
    lazily refreshes a sorted cache; Python performs no metric arithmetic.
    """

    def __init__(self, cutoff: float = 0.05, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(float(cutoff), nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "HistoricalValueAtRisk":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "HistoricalValueAtRisk":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "HistoricalValueAtRisk":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "HistoricalValueAtRisk":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

    def append(self, value: float) -> "HistoricalValueAtRisk":
        """Append one value in the selected domain and return this metric."""
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
