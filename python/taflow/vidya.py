"""Native Variable Index Dynamic Average interface."""

from typing import Any

import numpy as np

from ._native import StatefulVariableIndexDynamicAverage


class VariableIndexDynamicAverage:
    """Compute a CMO-modulated exponential average of close prices.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    length : int, default 14
        Number of recent changes used to determine directional weighting.
    alpha : float, optional
        EMA coefficient. When omitted, uses ``2 / (length + 1)``.
    """

    def __init__(
        self, close: Any | None = None, length: int = 14, alpha: float | None = None
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        length : object
            Input series, scalar parameter, or configuration value for this operation.
        alpha : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulVariableIndexDynamicAverage(length, alpha)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return the current average."""
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned average history."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest average value."""
        return self._state.value

    def reset(self) -> object:
        """Clear state and accumulated output."""
        self._state.reset()
        return self
