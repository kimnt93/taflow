"""Whole-history average drawdown episode metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import AverageDrawdown as _Native
from ._input import as_metric_series


class AverageDrawdown:
    """Compute mean trough magnitude across distinct drawdown episodes.

    The independent oracle is the pinned PerformanceAnalytics 2.1.0
    ``AverageDrawdown`` and
    ``findDrawdowns`` with geometric compounding. Wealth begins at a phantom
    value of one. Each contiguous run where percentage drawdown is strictly
    negative is one episode, and the episode contributes the positive magnitude
    of its deepest trough. A return to drawdown zero completes the episode; an
    unrecovered episode at the end is included using its current deepest trough.
    Warm-up ends after one usable return: a non-empty path with no negative
    episode returns ``0.0``. Empty and all-missing states return ``None``. The source convention is pinned to the
    CRAN 2.1.0 tarball with SHA-256
    ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. The P&L input method requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a conversion baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities and simple returns
    below -100% are rejected. Mutating lifecycle methods are fluent, native
    bulk work releases the GIL, and Rust owns all conversion and bounded O(1)
    streaming state.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "AverageDrawdown":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "AverageDrawdown":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "AverageDrawdown":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "AverageDrawdown":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

    def append(self, value: float) -> "AverageDrawdown":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "AverageDrawdown":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the positive mean episode depth, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "AverageDrawdown":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["AverageDrawdown"]
