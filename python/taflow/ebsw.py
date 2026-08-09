"""Native Even Better Sinewave interface."""

from typing import Any

import numpy as np

from ._native import StatefulEvenBetterSinewave


class EvenBetterSinewave:
    """Compute a causal detrended cycle oscillator from close prices.

    Parameters
    ----------
    close : array-like
        Initial aligned close history.
    length : int, default 40
        Nominal cycle length used to configure the oscillator state.
    """

    def __init__(
        self,
        close: Any,
        length: int = 40,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulEvenBetterSinewave(length)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> "EvenBetterSinewave":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.

        Returns
        -------
        EvenBetterSinewave
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "EvenBetterSinewave":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.

        Returns
        -------
        EvenBetterSinewave
            This indicator, for fluent chaining."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
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

    def reset(self) -> "EvenBetterSinewave":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        EvenBetterSinewave
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
