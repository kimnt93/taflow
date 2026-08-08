"""Descriptive stateful interface for a selectable moving average."""

from taflow._native import StatefulMa
from typing import Any

import numpy as np


class MovingAverage:
    """Incrementally compute any TA-Lib moving-average type

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    MovingAverage
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self, period: int = 30, moving_average_type: int = 0, values: Any | None = None
    ) -> None:
        """Create a selectable moving average with optional initial values."""
        self._state = StatefulMa(period, moving_average_type)
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
        self._state.append(float(value))
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
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned moving-average history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
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

    def reset(self) -> "MovingAverage":
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
