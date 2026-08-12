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
    initial capital. Mutating lifecycle operations are fluent.
    """

    def __init__(self, nan_policy: str = "omit") -> None:
        """Initialize an empty configured metric."""
        self._state = _Native(nan_policy)

    def from_returns(
        self, returns: Any, *, column: str | None = None
    ) -> "StabilityOfTimeSeries":
        """Append chronological decimal simple returns and return this metric."""
        self._state.from_returns(as_metric_series(returns, column=column))
        return self

    def from_log_returns(
        self, log_returns: Any, *, column: str | None = None
    ) -> "StabilityOfTimeSeries":
        """Append chronological log returns and return this metric."""
        self._state.from_log_returns(as_metric_series(log_returns, column=column))
        return self

    def from_equity(
        self, equity: Any, *, column: str | None = None
    ) -> "StabilityOfTimeSeries":
        """Append chronological positive equity levels and return this metric."""
        self._state.from_equity(as_metric_series(equity, column=column))
        return self

    def from_pnl(
        self,
        pnl: Any,
        initial_capital: float,
        *,
        column: str | None = None,
    ) -> "StabilityOfTimeSeries":
        """Append period P&L using required positive initial capital."""
        self._state.from_pnl(
            as_metric_series(pnl, column=column), float(initial_capital)
        )
        return self

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
