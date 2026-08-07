"""Descriptive stateful interface for the Average Directional Index."""

from taflow._native import StatefulAdx
from typing import Any


class AverageDirectionalIndex:
    """Incrementally compute Wilder's Average Directional Index."""

    def __init__(
        self,
        period: int = 14,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ):
        """Create the indicator and optionally process an initial history.

        Parameters are ``period`` (Wilder lookback), ``high``, ``low``, and
        ``close`` (aligned OHLC series).  The constructor returns no value;
        use ``extend`` for later history.
        """
        self._state = StatefulAdx(period)
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high, low, close):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
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
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
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
        return self._state.extend(high, low, close)

    @property
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
