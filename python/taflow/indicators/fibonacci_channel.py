from typing import Any

import numpy as np

from .._native import FibonacciChannel as _Native
from .._series import as_float64_series


class FibonacciChannel:
    """Causal Fibonacci chart levels from aligned high and low series.

    Rust owns the persistent extrema, level arithmetic, warm-up, history, and
    processed-bar count. Required input order is high then low. The 4 output
    arrays are ordered as: lower, 38.2% retracement, 61.8% retracement, upper. The first 2 positions are NaN.
    append, extend, and reset are fluent. The external mapping is Wickra 0.9.9
    FibChannel; TA-Lib has no equivalent chart-level function. TAFlow emits
    causal levels at each bar rather than a mutable chart annotation object.
    """

    def __init__(self, high: Any, low: Any) -> None:
        """Initialize and process required aligned chronological high/low data.

        Empty aligned series create fresh streaming state. Differing lengths
        raise ValueError before native state is mutated.
        """
        self._state = _Native()
        self.extend(high, low)

    def append(self, high: float, low: float) -> "FibonacciChannel":
        """Append one high/low observation and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "FibonacciChannel":
        """Append aligned high and low histories and return this adapter."""
        high_series = as_float64_series(high)
        low_series = as_float64_series(low)
        if len(high_series) != len(low_series):
            raise ValueError("high and low inputs must have equal lengths")
        self._state.extend(high_series, low_series)
        return self

    @property
    def value(self) -> tuple[float, float, float, float] | None:
        """Return the latest ordered native levels, or None during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        """Return the 4 aligned native output histories in documented order."""
        return self._state.compute()

    def reset(self) -> "FibonacciChannel":
        """Restore fresh native state without reallocating and return self."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
