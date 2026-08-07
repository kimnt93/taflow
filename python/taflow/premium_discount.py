"""Native premium/discount zone interface."""

from typing import Any

import numpy as np

from ._native import StatefulPremiumDiscount


class PremiumDiscount:
    """Classify closes against the midpoint of a rolling swing range.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    window : int, default 20
        Number of closes used to calculate the rolling high and low.
    """

    def __init__(self, close: Any | None = None, window: int = 20):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        window : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulPremiumDiscount(window)
        if close is not None:
            self.extend(close)

    def append(self, close: float):
        """Process one close and return `(zone, equilibrium)`."""
        return self._state.append(float(close))

    def extend(self, close: Any):
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self):
        """Return zone and equilibrium histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest zone and equilibrium pair."""
        return self._state.value

    def reset(self):
        """Clear rolling history and output."""
        self._state.reset()
        return self
