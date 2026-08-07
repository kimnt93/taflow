"""Native Fibonacci retracement interface."""

from typing import Any

import numpy as np

from ._native import StatefulFibonacciRetracement


class FibonacciRetracement:
    """Compute rolling Fibonacci levels from close-price ranges.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    window : int, default 120
        Rolling range used to determine high and low anchors.
    """

    def __init__(self, close: Any | None = None, window: int = 120):
        self._state = StatefulFibonacciRetracement(window)
        if close is not None:
            self.extend(close)

    def append(self, close: float):
        """Process one close and return seven retracement levels."""
        return self._state.append(float(close))

    def extend(self, close: Any):
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self):
        """Return seven aligned retracement level histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest seven retracement levels."""
        return self._state.value

    def reset(self):
        """Clear rolling history and level output."""
        self._state.reset()
        return self
