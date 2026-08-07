"""Descriptive stateful interface for fixed-parameter MACD."""

from taflow._native import StatefulMacdFix
from typing import Any


class MovingAverageConvergenceDivergenceFixed:
    """Incrementally compute TA-Lib's fixed 12/26 MACD variant."""

    def __init__(self, signal_period: int = 9, value: Any | None = None):
        """Create fixed MACD with an optional initial price series."""
        self._state = StatefulMacdFix(signal_period)
        if value is not None:
            self.extend(value)

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
