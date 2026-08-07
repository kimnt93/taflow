"""Persistent Ease of Movement."""

from typing import Any
import numpy as np
from ._native import EaseOfMovementOperator as _Native
from ._series import as_float64_series


class EaseOfMovement:
    """Stateful EaseOfMovement indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self, high: Any | None = None, low: Any | None = None, volume: Any | None = None
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

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

    def append(self, high: float, low: float, volume: float) -> object:
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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, volume)
        return self

    def extend(self, high: Any, low: Any, volume: Any) -> object:
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
        object
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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> object:
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
