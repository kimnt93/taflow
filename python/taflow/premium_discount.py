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

    def __init__(self, close: Any | None = None, window: int = 20) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        window : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulPremiumDiscount(window)
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return `(zone, equilibrium)`..

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
        """Process an aligned close history and return this indicator..

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

    def compute(self) -> object:
        """Return zone and equilibrium histories..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest zone and equilibrium pair..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Clear rolling history and output..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
