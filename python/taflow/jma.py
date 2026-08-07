"""Native Jurik-like adaptive moving-average interface."""

from typing import Any

import numpy as np

from ._native import StatefulJurikMovingAverage


class JurikMovingAverage:
    """Compute the documented public Jurik-like adaptive average.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    length : int, default 7
        Lookback controlling the base adaptive coefficient.
    phase : float, default 0
        Phase parameter retained by the public reconstruction interface.
    """

    def __init__(self, close: Any | None = None, length: int = 7, phase: float = 0):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        length : object
            Input series, scalar parameter, or configuration value for this operation.
        phase : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulJurikMovingAverage(length, phase)
        if close is not None:
            self.extend(close)

    def append(self, close: float):
        """Process one close and return the current adaptive average."""
        return self._state.append(float(close))

    def extend(self, close: Any):
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned adaptive-average history."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest adaptive average."""
        return self._state.value

    def reset(self):
        """Clear state and accumulated output."""
        self._state.reset()
        return self
