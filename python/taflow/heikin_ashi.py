"""Native Heikin-Ashi OHLC transform interface."""

from typing import Any

import numpy as np

from ._native import StatefulHeikinAshi


class HeikinAshi:
    """Compute causal transformed _open, high, low, and close values.

    Parameters
    ----------
    _open, high, low, close : array-like, optional
        Initial aligned OHLC history.
    """

    def __init__(
        self,
        _open: Any | None = None,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _open : object
            Open-price series or the current bar open.
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = StatefulHeikinAshi()
        if (
            _open is not None
            or high is not None
            or low is not None
            or close is not None
        ):
            self.extend(_open, high, low, close)

    def append(self, _open: float, high: float, low: float, close: float) -> object:
        """Process one OHLC bar and return transformed OHLC values

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.append(float(_open), float(high), float(low), float(close))

    def extend(self, _open: Any, high: Any, low: Any, close: Any) -> object:
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
        self._state.extend(
            np.asarray(_open, dtype=np.float64),
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
        )
        return self

    def compute(self) -> object:
        """Return transformed _open, high, low, and close histories

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest transformed OHLC tuple

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> object:
        """Clear previous-candle state and output history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
