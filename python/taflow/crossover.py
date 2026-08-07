"""Persistent crossover signal."""

from typing import Any
import numpy as np
from ._native import CrossoverOperator as _Native
from ._series import as_float64_series


class Crossover:
    """Stateful Crossover indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, left: Any | None = None, right: Any | None = None) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        left : object
            Input series, scalar parameter, or configuration value for this operation.
        right : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native()
        self.extend(left, right) if left is not None or right is not None else None

    def append(self, left: float, right: float) -> object:
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        left : object
            Input series, scalar parameter, or configuration value for this operation.
        right : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(left, right)
        return self

    def extend(self, left: Any, right: Any) -> object:
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        left : object
            Input series, scalar parameter, or configuration value for this operation.
        right : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(left), as_float64_series(right))
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
