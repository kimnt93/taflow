"""Native Even Better Sinewave interface."""

from typing import Any

import numpy as np

from ._native import StatefulEvenBetterSinewave


class EvenBetterSinewave:
    """Compute a causal detrended cycle oscillator from close prices.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    length : int, default 40
        Nominal cycle length used to configure the oscillator state.
    """

    def __init__(self, close: Any | None = None, length: int = 40) -> None:
        """Initialize this adapter and optionally process the supplied input series.

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

    def append(self, close: float) -> object:
        """Process one close and return the current oscillator value."""
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned oscillator history."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest oscillator value."""
        return self._state.value

    def reset(self) -> object:
        """Clear state and accumulated output."""
        self._state.reset()
        return self
