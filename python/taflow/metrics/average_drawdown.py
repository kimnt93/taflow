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
    or non-cumulative period P&L. The P&L factory requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a conversion baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities and simple returns
    below -100% are rejected. Mutating lifecycle methods are fluent, native
    bulk work releases the GIL, and Rust owns all conversion and bounded O(1)
    streaming state.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use AverageDrawdown.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "AverageDrawdown":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, initial_equity, nan_policy)
        return state.extend(values)

    @classmethod
    def from_returns(
        cls, returns: Any, *, nan_policy: str = "omit"
    ) -> "AverageDrawdown":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy)

    @classmethod
    def from_log_returns(
        cls, log_returns: Any, *, nan_policy: str = "omit"
    ) -> "AverageDrawdown":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy)

    @classmethod
    def from_equity(
        cls, equity: Any, *, nan_policy: str = "omit"
    ) -> "AverageDrawdown":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
    ) -> "AverageDrawdown":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl, "pnl", initial_equity=float(initial_equity), nan_policy=nan_policy
        )

    def append(self, value: float) -> "AverageDrawdown":
        """Append one value in the factory-selected domain and return this metric."""
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
