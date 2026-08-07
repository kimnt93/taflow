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
        """Append one value and update the native delayed result."""
        self._state.append(_input)
        return self

    def extend(self, _input: Any) -> "SignalDelay":
        """Process an aligned input series in native Rust."""
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned delayed series."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest delayed value."""
        return self._state.value

    def reset(self) -> "SignalDelay":
        """Reset the native state and accumulated history."""
        self._state.reset()
        return self
