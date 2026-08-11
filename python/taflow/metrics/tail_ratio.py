"""Exact whole-history return tail-ratio metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import TailRatio as _Native
from ._input import as_metric_series


class TailRatio:
    """Compute the magnitude ratio of the 95th and 5th return percentiles.

    TAFlow uses linear-interpolated exact quantiles. The external oracle and
    name mapping is Empyrical Reloaded 0.5.12 ``tail_ratio`` for defined
    results. Scalar warm-up is one usable normalized return. An empty state,
    or a zero-magnitude 5th percentile,
    returns ``None`` rather than emitting a non-finite ratio. Inputs may be
    decimal simple returns, log returns, positive equity levels, or
    non-cumulative period P&L with positive initial equity. The first equity
    level establishes a baseline and does not increment metric length.
    ``nan_policy`` is ``"omit"`` or ``"raise"``; infinities are rejected.
    Mutating lifecycle methods are fluent. Rust retains O(n) observations and
    lazily refreshes the sorted cache; a bulk extension sorts only after all
    accepted observations have been appended. Python performs no arithmetic.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use TailRatio.from_returns/from_equity/from_pnl/from_log_returns")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "TailRatio":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, initial_equity, nan_policy)
        return state.extend(values)

    @classmethod
    def from_returns(
        cls, returns: Any, *, nan_policy: str = "omit"
    ) -> "TailRatio":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy)

    @classmethod
    def from_log_returns(
        cls, log_returns: Any, *, nan_policy: str = "omit"
    ) -> "TailRatio":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy)

    @classmethod
    def from_equity(
        cls, equity: Any, *, nan_policy: str = "omit"
    ) -> "TailRatio":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
    ) -> "TailRatio":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl,
            "pnl",
            initial_equity=float(initial_equity),
            nan_policy=nan_policy,
        )

    def append(self, value: float) -> "TailRatio":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TailRatio":
        """Append a chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the current ratio, or ``None`` while undefined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current exact scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "TailRatio":
        """Clear retained observations, preserve configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns retained by Rust."""
        return len(self._state)


__all__ = ["TailRatio"]
