"""Descriptive stateful interface for fixed-parameter MACD."""

from taflow._native import StatefulMacdFix
from typing import Any

import numpy as np


class MovingAverageConvergenceDivergenceFixed:
    """Incrementally compute TA-Lib's fixed 12/26 MACD variant

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    MovingAverageConvergenceDivergenceFixed
        A persistent native-backed indicator adapter.
    """

    def __init__(self, signal_period: int = 9, values: Any | None = None) -> None:
        """Create fixed MACD with an optional initial price series."""
        self._state = StatefulMacdFix(signal_period)
        if values is not None:
            self.extend(values)

    def append(self, value: float) -> "MovingAverageConvergenceDivergenceFixed":
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

    def extend(self, values: Any) -> "MovingAverageConvergenceDivergenceFixed":
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

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned MACD, signal, and histogram histories

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

    def reset(self) -> "MovingAverageConvergenceDivergenceFixed":
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
