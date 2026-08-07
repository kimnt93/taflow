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

    def __init__(
        self, close: Any | None = None, length: int = 7, phase: float = 0
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        phase : object
            Hilbert transform phase parameter.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulJurikMovingAverage(length, phase)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return the current adaptive average."""
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process an aligned close history and return this indicator."""
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned adaptive-average history."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest adaptive average."""
        return self._state.value

    def reset(self) -> object:
        """Clear state and accumulated output."""
        self._state.reset()
        return self
