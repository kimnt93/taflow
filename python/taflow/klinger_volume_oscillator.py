"""Native Klinger volume oscillator interface."""

from typing import Any

import numpy as np

from ._native import StatefulKlingerVolumeOscillator


class KlingerVolumeOscillator:
    """Compute signed-volume force with fast/slow EMA and signal EMA.

    Parameters
    ----------
    high, low, close, volume : array-like, optional
        Initial aligned OHLCV history.
    fast, slow, signal : int, default 34, 55, 13
        EMA periods for force, baseline, and signal smoothing.
    """

    def __init__(self, high: Any | None = None, low: Any | None = None,
                 close: Any | None = None, volume: Any | None = None,
                 fast: int = 34, slow: int = 55, signal: int = 13):
        self._state = StatefulKlingerVolumeOscillator(fast, slow, signal)
        if close is not None:
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float):
        """Process one OHLCV bar and return oscillator and signal values."""
        return self._state.append(float(high), float(low), float(close), float(volume))

    def extend(self, high: Any, low: Any, close: Any, volume: Any):
        """Process aligned OHLCV history and return this indicator."""
        self._state.extend(np.asarray(high, dtype=np.float64),
                           np.asarray(low, dtype=np.float64),
                           np.asarray(close, dtype=np.float64),
                           np.asarray(volume, dtype=np.float64))
        return self

    def compute(self):
        """Return oscillator and signal histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest oscillator and signal pair."""
        return self._state.value

    def reset(self):
        """Clear EMA state and accumulated output."""
        self._state.reset()
        return self
