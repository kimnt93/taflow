"""Native session volume-profile level interface."""

from typing import Any

import numpy as np

from ._native import StatefulSessionVolumeLevels


class SessionVolumeLevels:
    """Compute point of control and value-area bounds per session.

    Parameters
    ----------
    high, low, close, volume : array-like, optional
        Initial aligned OHLCV history.
    anchor : array-like of bool, optional
        Session-boundary flags for the initial history.
    bins : int, default 24
        Number of fixed histogram price bins.
    value_area : float, default 0.7
        Fraction of session volume included in the value area.
    """

    def __init__(self, high: Any | None = None, low: Any | None = None,
                 close: Any | None = None, volume: Any | None = None,
                 anchor: Any | None = None, bins: int = 24,
                 value_area: float = 0.7):
        self._state = StatefulSessionVolumeLevels(bins, value_area)
        if close is not None:
            self.extend(high, low, close, volume, anchor)

    def append(self, high: float, low: float, close: float, volume: float,
               anchor: bool = False):
        """Process one OHLCV bar and return profile levels."""
        return self._state.append(float(high), float(low), float(close),
                                  float(volume), bool(anchor))

    def extend(self, high: Any, low: Any, close: Any, volume: Any,
               anchor: Any | None = None):
        """Process aligned OHLCV history and return this indicator."""
        close_array = np.asarray(close, dtype=np.float64)
        if anchor is None:
            anchor = np.zeros(close_array.shape, dtype=np.bool_)
        self._state.extend(np.asarray(high, dtype=np.float64),
                           np.asarray(low, dtype=np.float64), close_array,
                           np.asarray(volume, dtype=np.float64),
                           np.asarray(anchor, dtype=np.bool_))
        return self

    def compute(self):
        """Return point-of-control, value-area-high, and value-area-low histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest profile-level tuple."""
        return self._state.value

    def reset(self):
        """Clear profile histogram and session state."""
        self._state.reset()
        return self
