"""Persistent Ease of Movement."""

from typing import Any
import numpy as np
from ._native import EaseOfMovementOperator as _Native
from ._series import as_float64_series


class EaseOfMovement:
    """Persistent Ease of Movement.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `volume`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        volume: Any,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native()
        self._length = 0
        self.extend(high, low, volume)

    def append(self, high: float, low: float, volume: float) -> "EaseOfMovement":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(high), float(low), float(volume))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any, volume: Any) -> "EaseOfMovement":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        volume_values = as_float64_series(volume)
        if len({len(high_values), len(low_values), len(volume_values)}) != 1:
            raise ValueError("high, low, and volume must have equal lengths")
        self._state.extend(high_values, low_values, volume_values)
        self._length += len(high_values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
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

    def reset(self) -> "EaseOfMovement":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        return self._length
