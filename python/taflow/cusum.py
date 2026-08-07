"""CUSUM drift-detection accumulator on a change series."""

from typing import Any
import numpy as np
from ._native import CusumOperator as _Native
from ._series import as_float64_series


class Cusum:
    """Stateful Cusum indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, change: Any | None = None, threshold: float = 1.0):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        change : object
            Input series, scalar parameter, or configuration value for this operation.
        threshold : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(threshold)
        self.extend(change) if change is not None else None

    def append(self, change: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        change : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(change)
        return self

    def extend(self, change: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        change : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(change))
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
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
