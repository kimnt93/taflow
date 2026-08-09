"""Descriptive stateful interface for Acceleration Bands."""

from typing import Any

import numpy as np

from .._native import AccelerationBands as _NativeAccelerationBands
from .._series import as_float64_series


class AccelerationBands:
    """Incrementally compute upper, middle, and lower Acceleration Bands

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AccelerationBands
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        period: int = 20,
    ) -> None:
        """Create Acceleration Bands with optional aligned OHLC history."""
        self._state = _NativeAccelerationBands(period)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "AccelerationBands":
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

    def extend(self, high: Any, low: Any, close: Any) -> "AccelerationBands":
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
        arrays = tuple(as_float64_series(value) for value in (high, low, close))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, ...]:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "AccelerationBands":
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
