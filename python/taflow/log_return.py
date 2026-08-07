"""Persistent causal logarithmic return operator."""

from typing import Any
import numpy as np
from ._native import LogReturnOperator as _Native
from ._series import as_float64_series


class LogReturn:
    """Stateful LogReturn indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(self, timeperiod: int, _input: Any | None = None):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(timeperiod)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        _input : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(as_float64_series(_input))
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
