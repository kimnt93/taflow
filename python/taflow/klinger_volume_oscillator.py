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

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        volume: Any | None = None,
        fast: int = 34,
        slow: int = 55,
        signal: int = 13,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.
        fast : object
            Fast smoothing period in bars.
        slow : object
            Slow smoothing period in bars.
        signal : object
            Signal smoothing period in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulKlingerVolumeOscillator(fast, slow, signal)
        if close is not None:
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> object:
        """Process one OHLCV bar and return oscillator and signal values

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.append(float(high), float(low), float(close), float(volume))

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> object:
        """Process aligned OHLCV history and return this indicator

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
            np.asarray(volume, dtype=np.float64),
        )
        return self

    def compute(self) -> object:
        """Return oscillator and signal histories

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest oscillator and signal pair

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Clear EMA state and accumulated output

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
