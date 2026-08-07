"""Descriptive stateful interface for a selectable moving average."""

from taflow._native import StatefulMa
from typing import Any


class MovingAverage:
    """Incrementally compute any TA-Lib moving-average type."""

    def __init__(
        self, period: int = 30, moving_average_type: int = 0, values: Any | None = None
    ):
        """Create a selectable moving average with optional initial values."""
        self._state = StatefulMa(period, moving_average_type)
        if values is not None:
            self.extend(values)

    def append(self, value):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.append(value)

    def extend(self, values):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.extend(values)

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
