"""Native Parabolic Moving Average Stop interface."""

from typing import Any

import numpy as np

from ._native import StatefulParabolicMovingAverageStop


class ParabolicMovingAverageStop:
    """Compute an EMA/rolling-range stop and causal trend direction.

    Parameters
    ----------
    high, low, close : array-like, optional
        Initial aligned OHLC history.
    length : int, default 10
        EMA and true-range lookback.
    multiplier : float, default 3
        Stop distance multiplier.
    """

    def __init__(self, high: Any | None = None, low: Any | None = None,
                 close: Any | None = None, length: int = 10,
                 multiplier: float = 3.0):
        self._state = StatefulParabolicMovingAverageStop(length, multiplier)
        if close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float):
        """Process one OHLC bar and return stop and trend direction."""
        return self._state.append(float(high), float(low), float(close))

    def extend(self, high: Any, low: Any, close: Any):
        """Process aligned OHLC history and return this indicator."""
        self._state.extend(np.asarray(high, dtype=np.float64),
                           np.asarray(low, dtype=np.float64),
                           np.asarray(close, dtype=np.float64))
        return self

    def compute(self):
        """Return stop and trend histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest stop and trend pair."""
        return self._state.value

    def reset(self):
        """Clear EMA, range, and trend state."""
        self._state.reset()
        return self
