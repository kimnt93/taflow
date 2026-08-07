"""Native Tom DeMark Sequential interface."""

from typing import Any

import numpy as np

from ._native import StatefulTomDeMarkSequential


class TomDeMarkSequential:
    """Compute causal four-bar buy and sell setup counts.

    Parameters
    ----------
    close : array-like, optional
        Initial aligned close history.
    """

    def __init__(self, close: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulTomDeMarkSequential()
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return buy and sell counts."""
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> object:
        """Return buy and sell setup-count histories."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest buy and sell counts."""
        return self._state.value

    def reset(self) -> object:
        """Clear close history and setup counts."""
        self._state.reset()
        return self
