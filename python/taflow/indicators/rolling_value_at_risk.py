"""Public adapter for the native RollingValueAtRisk state."""

from typing import Any

import numpy as np

from .._native import RollingValueAtRisk as _Native
from .._series import as_float64_series


class RollingValueAtRisk:
    """Estimate lower-tail loss with a rolling empirical quantile.

    Rust owns the rolling window, arithmetic, warm-up, and aligned output
    history. This rolling series maps to Wickra ``ValueAtRisk`` 0.9.9; TA-Lib
    has no direct equivalent.

    Args:
        values: Initial chronological return or profit/loss series.
        timeperiod: Rolling window length. Defaults to 14.
        confidence: Tail confidence in the open interval ``(0, 1)``. Defaults to 0.95.

    Raises:
        ValueError: If the window or another configuration value is invalid.
    """

    def __init__(self, values: Any, timeperiod: int = 14, confidence: float = 0.95) -> None:
        """Initialize the state and process the supplied history."""
        self._state = _Native(int(timeperiod), float(confidence))
        self.extend(values)

    def append(self, value: float) -> "RollingValueAtRisk":
        """Append one observation and return this instance."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "RollingValueAtRisk":
        """Append a chronological series and return this instance."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest result, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned results, including warm-up ``NaN`` positions."""
        return self._state.compute()

    def reset(self) -> "RollingValueAtRisk":
        """Restore fresh-state behavior and return this instance."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of observations processed by Rust."""
        return len(self._state)


__all__ = ["RollingValueAtRisk"]
