"""Native-backed position-hold state."""

from typing import Any

import numpy as np

from ._native import PositionHoldOperator
from ._series import as_float64_series


class PositionHold:
    """Hold the most recently supplied position value.

    Parameters
    ----------
    _input : array-like, optional
        Initial position history.
    """

    def __init__(self, _input: Any | None = None) -> None:
        """Create the state and optionally process position history."""
        self._state = PositionHoldOperator()
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "PositionHold":
        """Append one position value and update the native result

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> "PositionHold":
        """Process an aligned position series in native Rust

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned held-position series

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest held position

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> "PositionHold":
        """Reset the native state and accumulated history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
