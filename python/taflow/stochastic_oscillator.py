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
        fast_k_period: object = 5,
        slow_k_period: object = 3,
        slow_k_average_type: object = 0,
        slow_d_period: object = 3,
        slow_d_average_type: object = 0,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

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
        self._values: list[tuple[float, ...]] = []
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> object:
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
        result = self._state.append(high, low, close)
        self._values.append((np.nan, np.nan) if result is None else tuple(result))
        return self

    def extend(self, high: object, low: object, close: object) -> object:
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
        result = self._state.extend(high, low, close)
        arrays = [np.asarray(item, dtype=np.float64) for item in result]
        self._values.extend(zip(*arrays))
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the aligned native output histories..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        if not self._values:
            empty = np.empty(0, dtype=np.float64)
            return tuple(empty.copy() for _ in range(2))
        return tuple(
            np.asarray(values, dtype=np.float64) for values in zip(*self._values)
        )

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
