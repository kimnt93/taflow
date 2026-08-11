"""Whole-history mean absolute drawdown metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import PainIndex as _Native
from ._input import as_metric_series


class PainIndex:
    """Compute mean absolute percentage drawdown over the wealth path.

    The independent oracle is the pinned PerformanceAnalytics 2.1.0
    ``PainIndex`` and its
    ``DrawdownPeak`` helper. Wealth begins at a phantom value of one; every
    normalized simple return compounds wealth, and each real observation's
    absolute percentage drawdown from the running peak contributes to the
    arithmetic mean. The phantom point establishes the peak but is not included
    in the divisor. The source convention is pinned to the CRAN 2.1.0 tarball
    with SHA-256
    ``fc801d39382818cd3a7052326b45d078302aef4d290c85dab83498ed4516d58d``.
    Warm-up ends after one usable return; empty and all-missing states yield
    ``None``.

    Inputs may be decimal simple returns, log returns, positive equity levels,
    or non-cumulative period P&L. The P&L factory requires positive initial
    equity and Rust performs causal capital conversion. The first equity level
    establishes a conversion baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities and simple returns
    below -100% are rejected. Mutating lifecycle methods are fluent, native
    bulk work releases the GIL, and Rust owns all conversion and allocation-free
    O(1) metric arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError(
            "use PainIndex.from_returns/from_equity/from_pnl/from_log_returns"
        )

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "PainIndex":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, initial_equity, nan_policy)
        return state.extend(values)

    @classmethod
    def from_returns(
        cls, returns: Any, *, nan_policy: str = "omit"
    ) -> "PainIndex":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy)

    @classmethod
    def from_log_returns(
        cls, log_returns: Any, *, nan_policy: str = "omit"
    ) -> "PainIndex":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy)

    @classmethod
    def from_equity(
        cls, equity: Any, *, nan_policy: str = "omit"
    ) -> "PainIndex":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
    ) -> "PainIndex":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl, "pnl", initial_equity=float(initial_equity), nan_policy=nan_policy
        )

    def append(self, value: float) -> "PainIndex":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "PainIndex":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return mean absolute drawdown, or ``None`` when no return was processed."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "PainIndex":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["PainIndex"]
