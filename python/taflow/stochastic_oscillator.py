"""Descriptive stateful interface for the Stochastic Oscillator."""

from taflow._native import StatefulStoch
from typing import Any

import numpy as np


class StochasticOscillator:
    """Incrementally compute aligned slow %K and slow %D

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    StochasticOscillator
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        fast_k_period: object = 5,
        slow_k_period: object = 3,
        slow_k_average_type: object = 0,
        slow_d_period: object = 3,
        slow_d_average_type: object = 0,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        fast_k_period : object
            Input parameter or configuration value for this operation.
        slow_k_period : object
            Input parameter or configuration value for this operation.
        slow_k_average_type : object
            Input parameter or configuration value for this operation.
        slow_d_period : object
            Input parameter or configuration value for this operation.
        slow_d_average_type : object
            Input parameter or configuration value for this operation.
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
        self._state = StatefulStoch(
            fast_k_period,
            slow_k_period,
            slow_k_average_type,
            slow_d_period,
            slow_d_average_type,
        )
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> "StochasticOscillator":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, close)
        return self

    def extend(self, high: object, low: object, close: object) -> "StochasticOscillator":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(high, low, close)
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "StochasticOscillator":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)
