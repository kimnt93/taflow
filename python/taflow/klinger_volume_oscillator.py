"""Native Klinger volume oscillator interface."""

from typing import Any

import numpy as np

from ._native import StatefulKlingerVolumeOscillator


class KlingerVolumeOscillator:
    """Compute signed-volume force with fast/slow EMA and signal EMA.

    Parameters
    ----------
    high, low, close, volume : array-like
        Initial aligned OHLCV history.
    fast, slow, signal : int, default 34, 55, 13
        EMA periods for force, baseline, and signal smoothing.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        fast: int = 34,
        slow: int = 55,
        signal: int = 13,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

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

    def append(self, high: float, low: float, close: float, volume: float) -> "KlingerVolumeOscillator":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        high : float
            Current high price.
        low : float
            Current low price.
        close : float
            Current close price.
        volume : float
            Current volume.

        Returns
        -------
        KlingerVolumeOscillator
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "KlingerVolumeOscillator":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        high : Any
            Chronological high price series.
        low : Any
            Chronological low price series.
        close : Any
            Chronological close price series.
        volume : Any
            Chronological volume series.

        Returns
        -------
        KlingerVolumeOscillator
            This indicator, for fluent chaining."""
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
            np.asarray(volume, dtype=np.float64),
        )
        return self

    def compute(self) -> object:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "KlingerVolumeOscillator":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        KlingerVolumeOscillator
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
