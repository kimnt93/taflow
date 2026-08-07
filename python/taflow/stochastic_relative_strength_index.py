"""Descriptive stateful interface for Stochastic RSI."""

from taflow._native import StatefulStochrsi
from typing import Any


class StochasticRelativeStrengthIndex:
    """Incrementally compute aligned stochastic-RSI fast %K and fast %D."""

    def __init__(
        self,
        time_period=14,
        fast_k_period=5,
        fast_d_period=3,
        fast_d_average_type=0,
        _input: Any | None = None,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        time_period : object
            Input series, scalar parameter, or configuration value for this operation.
        fast_k_period : object
            Input series, scalar parameter, or configuration value for this operation.
        fast_d_period : object
            Input series, scalar parameter, or configuration value for this operation.
        fast_d_average_type : object
            Input series, scalar parameter, or configuration value for this operation.
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = StatefulStochrsi(
            time_period,
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
        )
        if _input is not None:
            self.extend(_input)

    def append(self, _input):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.append(_input)

    def extend(self, _input):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.extend(_input)

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
