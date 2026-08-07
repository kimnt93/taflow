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

    def __init__(self, close: Any | None = None):
        self._state = StatefulTomDeMarkSequential()
        if close is not None:
            self.extend(close)

    def append(self, close: float):
        """Process one close and return buy and sell counts."""
        return self._state.append(float(close))

    def extend(self, close: Any):
        """Process aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self):
        """Return buy and sell setup-count histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest buy and sell counts."""
        return self._state.value

    def reset(self):
        """Clear close history and setup counts."""
        self._state.reset()
        return self
