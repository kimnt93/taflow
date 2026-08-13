from typing import Any

import numpy as np

from .._native import FibonacciConfluence as _Native
from .._series import as_float64_series


class FibonacciConfluence:
    """Find the densest recent cluster of causal Fibonacci retracements.

    Rust owns the persistent extrema, level arithmetic, warm-up, history, and
    processed-bar count. Required input order is high then low. Outputs are
    ``(price, strength)`` for the mean price and member count of the densest
    cluster across at most five confirmed swing legs. Both are NaN until three
    pivots provide two legs.
    append, extend, and reset are fluent. The external mapping is Wickra 0.9.9
    FibConfluence; TA-Lib has no equivalent chart-level function. TAFlow emits
    causal levels at each bar rather than a mutable chart annotation object.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.

        Empty aligned series create fresh streaming state. Differing lengths
        raise ValueError before native state is mutated.
        """
        self._state = _Native()

    def append(self, high: float, low: float) -> "FibonacciConfluence":
        """Append one high/low observation and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "FibonacciConfluence":
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

    def reset(self) -> "FibonacciConfluence":
        """Restore fresh native state without reallocating and return self."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
