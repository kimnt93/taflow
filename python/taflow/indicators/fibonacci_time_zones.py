from typing import Any

import numpy as np

from .._native import FibonacciTimeZones as _Native
from .._series import as_float64_series


class FibonacciTimeZones:
    """Track Fibonacci bar distances from the latest confirmed swing pivot.

    Rust owns the persistent extrema, level arithmetic, warm-up, history, and
    processed-bar count. Required input order is high then low. Outputs are
    ``(on_zone, bars_to_next)``: the first item is 1.0 on a Fibonacci offset
    and 0.0 otherwise, while the second gives the remaining bars to the next
    offset. Both are NaN until the first reversal confirms an anchor pivot.
    append, extend, and reset are fluent. The external mapping is Wickra 0.9.9
    FibTimeZones; TA-Lib has no equivalent chart-level function. TAFlow emits
    causal levels at each bar rather than a mutable chart annotation object.
    """

    def __init__(self, high: Any, low: Any) -> None:
        """Initialize and process required aligned chronological high/low data.

        Empty aligned series create fresh streaming state. Differing lengths
        raise ValueError before native state is mutated.
        """
        self._state = _Native()
        self.extend(high, low)

    def append(self, high: float, low: float) -> "FibonacciTimeZones":
        """Append one high/low observation and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "FibonacciTimeZones":
        """Append aligned high and low histories and return this adapter."""
        high_series = as_float64_series(high)
        low_series = as_float64_series(low)
        if len(high_series) != len(low_series):
            raise ValueError("high and low inputs must have equal lengths")
        self._state.extend(high_series, low_series)
        return self

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest ordered native levels, or None during warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return the 2 aligned native output histories in documented order."""
        return self._state.compute()

    def reset(self) -> "FibonacciTimeZones":
        """Restore fresh native state without reallocating and return self."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
