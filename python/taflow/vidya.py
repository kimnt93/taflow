"""Native Variable Index Dynamic Average interface."""

from typing import Any

import numpy as np

from ._native import StatefulVariableIndexDynamicAverage


class VariableIndexDynamicAverage:
    """Compute a CMO-modulated exponential average of close prices.

    Parameters
    ----------
    close : array-like
        Initial aligned close history.
    length : int, default 14
        Number of recent changes used to determine directional weighting.
    alpha : float, optional
        EMA coefficient. When omitted, uses ``2 / (length + 1)``.
    """

    def __init__(
        self,
        close: Any,
        length: int = 14,
        alpha: float | None = None,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        alpha : object
            Smoothing factor.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulVariableIndexDynamicAverage(length, alpha)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> "VariableIndexDynamicAverage":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        close : float
            Current close price.

        Returns
        -------
        VariableIndexDynamicAverage
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "VariableIndexDynamicAverage":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        close : Any
            Chronological close price series.

        Returns
        -------
        VariableIndexDynamicAverage
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

    def reset(self) -> "VariableIndexDynamicAverage":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        VariableIndexDynamicAverage
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
