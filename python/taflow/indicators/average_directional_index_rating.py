"""Descriptive stateful interface for the ADX Rating."""

from .._native import AverageDirectionalIndexRating as _NativeAverageDirectionalIndexRating
from typing import Any

import numpy as np

from .._series import as_float64_series


class AverageDirectionalIndexRating:
    """Incrementally compute the lag-averaged Average Directional Index

    Parameters
    ----------
    Construct with configuration values only; supply input series through ``extend``.

    Returns
    -------
    AverageDirectionalIndexRating
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        period: int = 14,
    ) -> None:
        """Create an empty configured ADXR state."""
        self._state = _NativeAverageDirectionalIndexRating(period)

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
        self._state.append(float(high), float(low), float(close))
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
        arrays = tuple(as_float64_series(series) for series in (high, low, close))
        if any(array.shape != arrays[0].shape for array in arrays[1:]):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
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
