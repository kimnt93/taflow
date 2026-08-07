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

    def __init__(self, close: Any | None = None, window: int = 120) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        window : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulFibonacciRetracement(window)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return seven retracement levels."""
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> object:
        """Return seven aligned retracement level histories."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest seven retracement levels."""
        return self._state.value

    def reset(self) -> object:
        """Clear rolling history and level output."""
        self._state.reset()
        return self
