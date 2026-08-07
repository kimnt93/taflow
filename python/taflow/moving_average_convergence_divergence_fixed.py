"""Descriptive stateful interface for fixed-parameter MACD."""

from taflow._native import StatefulMacdFix
from typing import Any

import numpy as np


class MovingAverageConvergenceDivergenceFixed:
    """Incrementally compute TA-Lib's fixed 12/26 MACD variant."""

    def __init__(self, signal_period: int = 9, value: Any | None = None) -> None:
        """Create fixed MACD with an optional initial price series."""
        self._state = StatefulMacdFix(signal_period)
        self._values: list[tuple[float, float, float]] = []
        if value is not None:
            self.extend(value)

    def append(self, value: float) -> "MovingAverageConvergenceDivergenceFixed":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input value processed at each bar.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(float(value))
        self._values.append(
            (np.nan, np.nan, np.nan) if result is None else tuple(result)
        )
        return self

    def extend(self, values: Any) -> "MovingAverageConvergenceDivergenceFixed":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input values processed in chronological order.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(values)
        arrays = [np.asarray(item, dtype=np.float64) for item in result]
        self._values.extend(zip(*arrays))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned MACD, signal, and histogram histories."""
        if not self._values:
            empty = np.empty(0, dtype=np.float64)
            return empty.copy(), empty.copy(), empty.copy()
        return tuple(
            np.asarray(values, dtype=np.float64) for values in zip(*self._values)
        )

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "MovingAverageConvergenceDivergenceFixed":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
