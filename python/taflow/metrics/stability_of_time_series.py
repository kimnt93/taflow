"""Cumulative-log-return trend stability metric."""

from __future__ import annotations
from typing import Any
from .._native.metrics import StabilityOfTimeSeries as _Native
from ._input import as_metric_series


class StabilityOfTimeSeries:
    """Compute R-squared of cumulative log returns against observation index.

    The independent oracle is Empyrical Reloaded 0.5.12
    ``stability_of_timeseries`` after
    omitting missing observations. Warm-up requires two usable returns; a
    constant cumulative path or a total-loss return yields ``None``. Rust owns
    the O(1) regression state and all semantic conversion. Inputs are decimal
    returns, log returns, positive equity levels, or period P&L with positive
    initial equity. Mutating lifecycle operations are fluent.
    """

    def __init__(self) -> None:
        """Reject ambiguous construction; use a semantic ``from_*`` factory."""
        raise TypeError("use StabilityOfTimeSeries.from_returns/from_equity/from_pnl/from_log_returns")

    @classmethod
    def _create(cls, values: Any, mode: str, *, initial_equity: float | None = None, nan_policy: str = "omit", column: str | None = None) -> "StabilityOfTimeSeries":
        state = cls.__new__(cls)
        state._state = _Native(mode, initial_equity=initial_equity, nan_policy=nan_policy)
        return state.extend(values, column=column)

    @classmethod
    def from_returns(cls, returns: Any, *, nan_policy: str = "omit", column: str | None = None) -> "StabilityOfTimeSeries":
        """Construct from chronological decimal simple returns."""
        return cls._create(returns, "returns", nan_policy=nan_policy, column=column)

    @classmethod
    def from_log_returns(cls, log_returns: Any, *, nan_policy: str = "omit", column: str | None = None) -> "StabilityOfTimeSeries":
        """Construct from chronological log returns converted by Rust."""
        return cls._create(log_returns, "log_returns", nan_policy=nan_policy, column=column)

    @classmethod
    def from_equity(cls, equity: Any, *, nan_policy: str = "omit", column: str | None = None) -> "StabilityOfTimeSeries":
        """Construct from chronological positive equity or adjusted-price levels."""
        return cls._create(equity, "equity", nan_policy=nan_policy, column=column)

    @classmethod
    def from_pnl(cls, pnl: Any, *, initial_equity: float, nan_policy: str = "omit", column: str | None = None) -> "StabilityOfTimeSeries":
        """Construct from period P&L and required positive initial equity."""
        return cls._create(pnl, "pnl", initial_equity=float(initial_equity), nan_policy=nan_policy, column=column)

    def append(self, value: float) -> "StabilityOfTimeSeries":
        """Append one value in the selected domain and return this metric."""
        self._state.append(float(value)); return self

    def extend(self, values: Any, *, column: str | None = None) -> "StabilityOfTimeSeries":
        """Append a chronological series and return this metric."""
        self._state.extend(as_metric_series(values, column=column)); return self

    @property
    def value(self) -> float | None:
        """Return current R-squared, or ``None`` until it is defined."""
        return self._state.value

    def compute(self) -> float | None:
        """Return the current scalar without replaying processed input."""
        return self._state.compute()

    def reset(self) -> "StabilityOfTimeSeries":
        """Clear observations, preserve input settings, and return this metric."""
        self._state.reset(); return self

    def __len__(self) -> int:
        """Return the usable normalized-return count delegated to Rust."""
        return len(self._state)


__all__ = ["StabilityOfTimeSeries"]
