"""Native SSL Channel interface."""

from typing import Any

import numpy as np

from ._native import StatefulSmoothedTrendChannel


class SmoothedTrendChannel:
    """Compute rolling high/low averages with causal trend-side ordering.

    Parameters
    ----------
    high, low, close : array-like, optional
        Initial aligned OHLC history.
    length : int, default 10
        Rolling average period.
    """

    def __init__(self, high: Any | None = None, low: Any | None = None,
                 close: Any | None = None, length: int = 10):
        self._state = StatefulSmoothedTrendChannel(length)
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close)

    def append(self, high: float, low: float, close: float):
        """Process one OHLC bar and return lower/upper channel values."""
        return self._state.append(float(high), float(low), float(close))

    def extend(self, high: Any, low: Any, close: Any):
        """Process aligned OHLC history and return this indicator."""
        self._state.extend(np.asarray(high, dtype=np.float64),
                           np.asarray(low, dtype=np.float64),
                           np.asarray(close, dtype=np.float64))
        return self

    def compute(self):
        """Return lower and upper channel histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest channel pair, or ``None`` if not warm."""
        return self._state.value

    def reset(self):
        """Clear state and accumulated channel history."""
        self._state.reset()
        return self
