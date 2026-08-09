"""Native Fibonacci retracement interface."""

from typing import Any

import numpy as np

from ._native import StatefulFibonacciRetracement


class FibonacciRetracement:
    """Compute rolling Fibonacci levels from close-price ranges.

    Parameters
    ----------
    close : array-like
        Initial aligned close history.
    window : int, default 120
        Rolling range used to determine high and low anchors.
    """

    def __init__(
        self,
        close: Any,
        window: int = 120,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        window : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulFibonacciRetracement(window)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> "FibonacciRetracement":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.

        Returns
        -------
        FibonacciRetracement
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "FibonacciRetracement":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.

        Returns
        -------
        FibonacciRetracement
            This indicator, for fluent chaining."""
        self._state.extend(np.asarray(close, dtype=np.float64))
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

    def reset(self) -> "FibonacciRetracement":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        FibonacciRetracement
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
