"""Native-backed causal signal delay state."""

from typing import Any

import numpy as np

from ._native import SignalDelayOperator
from ._series import as_float64_series


class SignalDelay:
    """Delay a scalar series by a fixed number of bars.

    Parameters
    ----------
    timeperiod : int
        Number of bars to delay.
    _input : array-like, optional
        Initial input history.
    """

    def __init__(self, timeperiod: int, _input: Any | None = None) -> None:
        """Create the delay state and optionally process input history."""
        self._state = SignalDelayOperator(timeperiod)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "SignalDelay":
        """Append one value and update the native delayed result..

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

    def extend(self, _input: Any) -> "SignalDelay":
        """Process an aligned input series in native Rust..

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
        """Return the aligned delayed series..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest delayed value..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> "SignalDelay":
        """Reset the native state and accumulated history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
