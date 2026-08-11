"""Whole-history root-mean-square drawdown metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import UlcerIndex as _Native
from ._input import as_metric_series


class UlcerIndex:
    """Compute RMS percentage drawdown over the complete wealth path.

    This whole-history portfolio metric is distinct from the rolling technical
    indicator exported as ``taflow.UlcerIndex``. A phantom wealth level of one
    precedes the first normalized return. For ``n >= 2`` usable returns, the
    result is ``sqrt(sum(drawdown**2) / (n - 1))``. The frozen external oracle
    is QuantStats 0.0.81 ``ulcer_index``, including its sample-style divisor.
    The positive result is ``None`` during the zero- and one-return warm-up.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. The P&L factory requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a baseline and does not increment the metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities and simple returns
    below -100% are rejected. Mutating lifecycle methods are fluent, bulk work
    releases the GIL, and all financial arithmetic and state live in Rust.
    PerformanceAnalytics' Ulcer Index is a formula cross-check, while the
    QuantStats divisor is the frozen numerical contract.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use UlcerIndex.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "UlcerIndex":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, initial_equity, nan_policy)
        return state.extend(values)

    @classmethod
    def from_returns(
        cls, returns: Any, *, nan_policy: str = "omit"
    ) -> "UlcerIndex":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy)

    @classmethod
    def from_log_returns(
        cls, log_returns: Any, *, nan_policy: str = "omit"
    ) -> "UlcerIndex":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy)

    @classmethod
    def from_equity(
        cls, equity: Any, *, nan_policy: str = "omit"
    ) -> "UlcerIndex":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
    ) -> "UlcerIndex":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl, "pnl", initial_equity=float(initial_equity), nan_policy=nan_policy
        )

    def append(self, value: float) -> "UlcerIndex":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "UlcerIndex":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return positive RMS drawdown, or ``None`` before two usable returns."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "UlcerIndex":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["UlcerIndex"]
