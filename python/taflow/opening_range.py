"""Native opening-range breakout interface."""

from typing import Any

import numpy as np

from ._native import StatefulOpeningRange


class OpeningRange:
    """Track session opening high/low levels and breakout direction.

    Parameters
    ----------
    high, low, close : array-like, optional
        Initial aligned OHLC history.
    anchor : array-like of bool, optional
        Session-boundary flags for the initial history.
    bars : int, default 30
        Number of bars used to form each opening range.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        anchor: Any | None = None,
        bars: int = 30,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        anchor : object
            Boolean series marking reset or anchor bars.
        bars : object
            Number of bars in the opening range.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulOpeningRange(bars)
        if high is not None or low is not None or close is not None:
            self.extend(high, low, close, anchor)

    def append(
        self, high: float, low: float, close: float, anchor: bool = False
    ) -> object:
        """Process one bar and return opening high, low, and breakout flag

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.append(float(high), float(low), float(close), bool(anchor))

    def extend(
        self, high: Any, low: Any, close: Any, anchor: Any | None = None
    ) -> object:
        """Process aligned OHLC history and return this indicator

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        close_array = np.asarray(close, dtype=np.float64)
        if anchor is None:
            anchor = np.zeros(close_array.shape, dtype=np.bool_)
        self._state.extend(
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            close_array,
            np.asarray(anchor, dtype=np.bool_),
        )
        return self

    def compute(self) -> object:
        """Return opening highs, lows, and breakout flags

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest opening range tuple

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Clear current session and output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
