"""Persistent Force Index."""

from typing import Any
import numpy as np
from ._native import ForceIndexOperator as _Native
from ._series import as_float64_series


class ForceIndex:
    """Stateful ForceIndex indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, close: Any | None = None, volume: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        volume : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native()
        self.extend(close, volume) if close is not None or volume is not None else None

    def append(self, close: float, volume: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        volume : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(close, volume)
        return self

    def extend(self, close: Any, volume: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        volume : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(close), as_float64_series(volume))
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
