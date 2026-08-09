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
    _input : array-like
        Initial input history.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 1,
    ) -> None:
        """Create the delay state and process input history."""
        self._state = SignalDelayOperator(timeperiod)
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float) -> "SignalDelay":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        _input : float
            Current input.

        Returns
        -------
        SignalDelay
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> "SignalDelay":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        _input : Any
            Chronological input series.

        Returns
        -------
        SignalDelay
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

    def reset(self) -> "SignalDelay":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        SignalDelay
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
