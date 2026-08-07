"""Persistent Triple Exponential Rate of Change (TRIX)."""

from typing import Any
import numpy as np
from ._native import TripleExponentialRateOfChange as _Native
from ._series import as_float64_series


class TripleExponentialRateOfChange:
    """Stateful TripleExponentialRateOfChange indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, _input: Any | None = None, timeperiod: int = 30) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(timeperiod)
        if _input is not None:
            self.extend(_input)

    def append(self, value: float) -> "TripleExponentialRateOfChange":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        value : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "TripleExponentialRateOfChange":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        values : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(values))
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
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "TripleExponentialRateOfChange":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Execute the __len__ operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return len(self._state)
