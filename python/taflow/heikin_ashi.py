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
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _open : object
            Input series, scalar parameter, or configuration value for this operation.
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulHeikinAshi()
        if (
            _open is not None
            or high is not None
            or low is not None
            or close is not None
        ):
            self.extend(_open, high, low, close)

    def append(self, _open: float, high: float, low: float, close: float):
        """Process one OHLC bar and return transformed OHLC values."""
        return self._state.append(float(_open), float(high), float(low), float(close))

    def extend(self, _open: Any, high: Any, low: Any, close: Any):
        """Process aligned OHLC history and return this indicator."""
        self._state.extend(
            np.asarray(_open, dtype=np.float64),
            np.asarray(high, dtype=np.float64),
            np.asarray(low, dtype=np.float64),
            np.asarray(close, dtype=np.float64),
        )
        return self

    def compute(self):
        """Return transformed _open, high, low, and close histories."""
        return self._state.compute()

    @property
    def value(self):
        """Return the latest transformed OHLC tuple."""
        return self._state.value

    def reset(self):
        """Clear previous-candle state and output history."""
        self._state.reset()
        return self
