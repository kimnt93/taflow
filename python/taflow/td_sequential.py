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
            Close-price series or the current bar close.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulTomDeMarkSequential()
        if close is not None:
            self.extend(close)

    def append(self, close: float) -> object:
        """Process one close and return buy and sell counts

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
        """Process aligned close history and return this indicator

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
        """Return buy and sell setup-count histories

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest buy and sell counts

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Clear close history and setup counts

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
