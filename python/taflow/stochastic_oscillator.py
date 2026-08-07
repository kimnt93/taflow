"""Descriptive stateful interface for the Stochastic Oscillator."""

from taflow._native import StatefulStoch
from typing import Any


class StochasticOscillator:
    """Incrementally compute aligned slow %K and slow %D."""

    def __init__(
        self,
        fast_k_period=5,
        slow_k_period=3,
        slow_k_average_type=0,
        slow_d_period=3,
        slow_d_average_type=0,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        fast_k_period : object
            Input series, scalar parameter, or configuration value for this operation.
        slow_k_period : object
            Input series, scalar parameter, or configuration value for this operation.
        slow_k_average_type : object
            Input series, scalar parameter, or configuration value for this operation.
        slow_d_period : object
            Input series, scalar parameter, or configuration value for this operation.
        slow_d_average_type : object
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
        self._state = StatefulStoch(
            fast_k_period,
            slow_k_period,
            slow_k_average_type,
            slow_d_period,
            slow_d_average_type,
        )
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
