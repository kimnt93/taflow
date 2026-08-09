"""Native-backed position-hold state."""

from typing import Any

import numpy as np

from ._native import PositionHoldOperator
from ._series import as_float64_series


class PositionHold:
    """Hold the most recently supplied position value.

    Parameters
    ----------
    _input : array-like
        Initial position history.
    """

    def __init__(
        self,
        _input: Any,
    ) -> None:
        """Create the state and process position history."""
        self._state = PositionHoldOperator()
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "PositionHold":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _input : float
            Current input.

        Returns
        -------
        PositionHold
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> "PositionHold":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _input : Any
            Chronological input series.

        Returns
        -------
        PositionHold
            This indicator, for fluent chaining."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "PositionHold":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        PositionHold
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
