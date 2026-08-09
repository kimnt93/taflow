"""Descriptive stateful interface for the ADX Rating."""

from taflow._native import StatefulAdxr
from typing import Any

import numpy as np


class AverageDirectionalIndexRating:
    """Incrementally compute the lag-averaged Average Directional Index

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AverageDirectionalIndexRating
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        period: int = 14,
    ) -> None:
        """Create ADXR with an optional aligned high/low/close history."""
        self._state = StatefulAdxr(period)
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high: object, low: object, close: object) -> "AverageDirectionalIndexRating":
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

    def extend(self, high: object, low: object, close: object) -> "AverageDirectionalIndexRating":
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

    def compute(self) -> np.ndarray:
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

    def reset(self) -> "AverageDirectionalIndexRating":
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
