"""Native Laguerre Relative Strength Index interface."""

from typing import Any

import numpy as np

from ._native import StatefulLaguerreRelativeStrengthIndex


class LaguerreRelativeStrengthIndex:
    """Compute Ehlers' four-stage Laguerre RSI on close prices.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    gamma : float, default 0.5
        Laguerre smoothing coefficient in the interval ``[0, 1)``.
    """

    def __init__(self, close: Any | None = None, gamma: float = 0.5) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        gamma : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulLaguerreRelativeStrengthIndex(gamma)
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
        """Return the latest oscillator value, or ``None`` if empty."""
        return self._state.value

    def reset(self) -> object:
        """Clear state and accumulated output."""
        self._state.reset()
        return self
