"""Persistent volume-weighted moving average."""

from typing import Any
import numpy as np
from ._native import VwmaOperator as _Native
from ._series import as_float64_series


class VolumeWeightedMovingAverage:
    """Persistent volume-weighted moving average.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `price`, `volume`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        price: Any,
        volume: Any,
        timeperiod: int = 10,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        timeperiod : object
            Trailing window length in bars.
        price : object
            Price series or the current price observation.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(price, volume)

    def append(self, price: float, volume: float) -> "VolumeWeightedMovingAverage":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        price : object
            Price series or the current price observation.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(price), float(volume))
        self._length += 1
        return self

    def extend(self, price: Any, volume: Any) -> "VolumeWeightedMovingAverage":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        price : object
            Price series or the current price observation.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        price_values = as_float64_series(price)
        volume_values = as_float64_series(volume)
        if len(price_values) != len(volume_values):
            raise ValueError("price and volume input series must have equal length")
        self._state.extend(price_values, volume_values)
        self._length += len(price_values)
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

    def __len__(self) -> int:
        """Return the number of paired observations consumed by this state."""
        return self._length

    def reset(self) -> "VolumeWeightedMovingAverage":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self
