"""Descriptive stateful interface for Acceleration Bands."""

from taflow._native import StatefulAccbands
from typing import Any


class AccelerationBands:
    """Incrementally compute upper, middle, and lower Acceleration Bands."""

    def __init__(
        self,
        period: int = 20,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ):
        """Create Acceleration Bands with optional aligned OHLC history."""
        self._state = StatefulAccbands(period)
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
