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
            Close-price series or the current bar close.
        length : object
            Indicator lookback or state length in bars.
        mom : object
            Momentum lookback in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulRelativeMomentumIndex(length, mom)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return the current RMI value when warm

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.append(float(close))

    def extend(self, close: Any) -> object:
        """Process an aligned close history and return this indicator

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(np.asarray(close, dtype=np.float64))
        return self

    def compute(self) -> np.ndarray:
        """Return all processed values with NaN warm-up entries

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the most recently computed value, or ``None`` if cold

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Clear state and previously computed output values

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
