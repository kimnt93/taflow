"""Descriptive stateful interface for a selectable moving average."""

from taflow._native import StatefulMa
from typing import Any

import numpy as np


class MovingAverage:
    """Incrementally compute any TA-Lib moving-average type."""

    def __init__(
        self, period: int = 30, moving_average_type: int = 0, values: Any | None = None
    ) -> None:
        """Create a selectable moving average with optional initial values."""
        self._state = StatefulMa(period, moving_average_type)
        self._values: list[float] = []
        if values is not None:
            self.extend(values)

    def append(self, value: float) -> "MovingAverage":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input value processed at each bar.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.append(float(value))
        self._values.append(np.nan if result is None else float(result))
        return self

    def extend(self, values: Any) -> "MovingAverage":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input values processed in chronological order.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        result = self._state.extend(values)
        self._values.extend(np.asarray(result, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned moving-average history."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "MovingAverage":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._values.clear()
        return self
