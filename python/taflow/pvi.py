"""Persistent Positive Volume Index."""

from typing import Any
import numpy as np
from ._native import PositiveVolumeIndexOperator as _Native
from ._series import as_float64_series


class PositiveVolumeIndex:
    """Persistent Positive Volume Index.

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
        self.extend(close, volume) if close is not None or volume is not None else None

    def append(self, close: float, volume: float) -> "PositiveVolumeIndex":
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
        self._state.append(close, volume)
        return self

    def extend(self, close: Any, volume: Any) -> "PositiveVolumeIndex":
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
        self._state.extend(as_float64_series(close), as_float64_series(volume))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "PositiveVolumeIndex":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
