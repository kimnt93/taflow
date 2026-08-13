"""Descriptive stateful interface for Parabolic SAR."""

from typing import Any

import numpy as np

from .._native import ParabolicSar as _NativeParabolicSar
from .._series import as_float64_series


class ParabolicSar:
    """Incrementally compute Parabolic SAR from high/low bars

    Parameters
    ----------
    Construct with configuration values only; supply input series through ``extend``.

    Returns
    -------
    ParabolicSar
        A persistent native-backed indicator adapter.
    """

    def __init__(self, acceleration: float = 0.02, maximum: float = 0.2) -> None:
        """Create an empty configured Parabolic SAR state."""
        self._state = _NativeParabolicSar(acceleration, maximum)

    def append(self, high: float, low: float) -> "ParabolicSar":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "ParabolicSar":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        high_array = as_float64_series(high)
        low_array = as_float64_series(low)
        if len(high_array) != len(low_array):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(high_array, low_array)
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

    def reset(self) -> "ParabolicSar":
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
