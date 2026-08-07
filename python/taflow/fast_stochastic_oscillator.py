"""Descriptive stateful interface for the Fast Stochastic Oscillator."""

from taflow._native import StatefulStochf
from typing import Any

import numpy as np


class FastStochasticOscillator:
    """Incrementally compute aligned fast %K and fast %D."""

    def __init__(
        self,
        fast_k_period: int = 5,
        fast_d_period: int = 3,
        fast_d_average_type: int = 0,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
    ):
        """Create fast stochastic with optional aligned OHLC history."""
        self._state = StatefulStochf(
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
        )
        self._values: list[tuple[float, ...]] = []
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
        result = self._state.append(high, low, close)
        self._values.append((np.nan, np.nan) if result is None else tuple(result))
        return self

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
        result = self._state.extend(high, low, close)
        arrays = [np.asarray(item, dtype=np.float64) for item in result]
        self._values.extend(zip(*arrays))
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the aligned native output histories."""
        if not self._values:
            empty = np.empty(0, dtype=np.float64)
            return tuple(empty.copy() for _ in range(2))
        return tuple(
            np.asarray(values, dtype=np.float64) for values in zip(*self._values)
        )

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
        self._values.clear()
        return self
