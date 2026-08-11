"""Compounded whole-history return metric."""

from __future__ import annotations

from typing import Any

from .._native.metrics import TotalReturn as _Native
from ._input import as_metric_series


class TotalReturn:
    """Compute compounded total return over a persistent observation stream.

    The formula is ``product(1 + return) - 1``. The independent oracle is
    Empyrical Reloaded 0.5.12 ``cum_returns_final``. Inputs may be decimal simple returns, log
    returns, positive equity levels, or non-cumulative period P&L. The P&L
    factory requires positive initial equity and Rust performs causal capital
    conversion. The first equity level establishes a baseline and does not
    increment the metric length. ``nan_policy`` is ``"omit"`` or ``"raise"``;
    infinities and simple returns below -100% are rejected. There is no warm-up
    beyond the first usable return; empty state returns ``None``. Mutating lifecycle methods are fluent and all financial arithmetic
    and state live in Rust.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use TotalReturn.from_returns/from_equity/from_pnl/from_log_returns")

    @classmethod
    def _create(
        cls,
        values: Any,
        input_mode: str,
        *,
        initial_equity: float | None = None,
        nan_policy: str = "omit",
    ) -> "TotalReturn":
        state = cls.__new__(cls)
        state._state = _Native(input_mode, initial_equity, nan_policy)
        return state.extend(values)

    @classmethod
    def from_returns(cls, returns: Any, *, nan_policy: str = "omit") -> "TotalReturn":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy)

    @classmethod
    def from_log_returns(
        cls, log_returns: Any, *, nan_policy: str = "omit"
    ) -> "TotalReturn":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy)

    @classmethod
    def from_equity(cls, equity: Any, *, nan_policy: str = "omit") -> "TotalReturn":
        """Construct from positive chronological equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy)

    @classmethod
    def from_pnl(
        cls,
        pnl: Any,
        *,
        initial_equity: float,
        nan_policy: str = "omit",
    ) -> "TotalReturn":
        """Construct from non-cumulative period P&L and positive initial equity."""
        return cls._create(
            pnl, "pnl", initial_equity=float(initial_equity), nan_policy=nan_policy
        )

    def append(self, value: float) -> "TotalReturn":
        """Append one value in the factory-selected domain and return this metric."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TotalReturn":
        """Append one chronological series in the selected domain and return this metric."""
        self._state.extend(as_metric_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return current compounded return, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "TotalReturn":
        """Clear observations, preserve input configuration, and return this metric."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of usable normalized returns processed by Rust."""
        return len(self._state)


__all__ = ["TotalReturn"]
