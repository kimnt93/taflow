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
        (
            self.extend(high, low, volume)
            if high is not None or low is not None or volume is not None
            else None
        )

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
        self._state.append(high, low, volume)
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
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(volume)
        )
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

    def reset(self) -> "EaseOfMovement":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
