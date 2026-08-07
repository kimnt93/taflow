"""Native Relative Momentum Index interface."""

from typing import Any

import numpy as np

from ._native import StatefulRelativeMomentumIndex


class RelativeMomentumIndex:
    """Compute Wilder-smoothed momentum gains over an aligned price series.

    Parameters
    ----------
    close : array-like, optional
        Initial price history. Values are processed in input order.
    length : int, default 14
        Number of momentum observations used for Wilder smoothing.
    mom : int, default 5
        Lag, in bars, used to measure each momentum change.
    """

    def __init__(
        self, close: Any | None = None, length: int = 14, mom: int = 5
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        length : object
            Input series, scalar parameter, or configuration value for this operation.
        mom : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulRelativeMomentumIndex(length, mom)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return the current RMI value when warm."""
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return all processed values with NaN warm-up entries."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the most recently computed value, or ``None`` if cold."""
        return self._state.value

    def reset(self) -> object:
        """Clear state and previously computed output values."""
        self._state.reset()
        return self
