"""Persistent Force Index."""

from typing import Any
import numpy as np
from ._native import ForceIndexOperator as _Native
from ._series import as_float64_series


class ForceIndex:
    """Persistent Force Index.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `close`, `volume`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        close: Any,
        volume: Any,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native()
        self._length = 0
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "ForceIndex":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(close), float(volume))
        self._length += 1
        return self

    def extend(self, close: Any, volume: Any) -> "ForceIndex":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        close_values = as_float64_series(close)
        volume_values = as_float64_series(volume)
        if len(close_values) != len(volume_values):
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_values, volume_values)
        self._length += len(close_values)
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

    def reset(self) -> "ForceIndex":
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
