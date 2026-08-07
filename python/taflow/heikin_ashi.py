"""Native Heikin-Ashi OHLC transform interface."""

from typing import Any

import numpy as np

from ._native import StatefulHeikinAshi


class HeikinAshi:
    """Compute causal transformed open, high, low, and close values.

    Parameters
    ----------
    open, high, low, close : array-like, optional
        Initial aligned OHLC history.
    """

    def __init__(self, open: Any | None = None, high: Any | None = None,
                 low: Any | None = None, close: Any | None = None):
        self._state = StatefulHeikinAshi()
        if open is not None or high is not None or low is not None or close is not None:
            self.extend(open, high, low, close)

    def append(self, open: float, high: float, low: float, close: float):
        """Process one OHLC bar and return transformed OHLC values."""
        return self._state.append(float(open), float(high), float(low), float(close))

    def extend(self, open: Any, high: Any, low: Any, close: Any):
        """Process aligned OHLC history and return this indicator."""
        self._state.extend(np.asarray(open, dtype=np.float64),
                           np.asarray(high, dtype=np.float64),
                           np.asarray(low, dtype=np.float64),
                           np.asarray(close, dtype=np.float64))
        return self

    def compute(self):
        """Return transformed open, high, low, and close histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest transformed OHLC tuple."""
        return self._state.value

    def reset(self):
        """Clear previous-candle state and output history."""
        self._state.reset()
        return self
