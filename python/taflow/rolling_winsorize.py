"""Persistent causal rolling winsorization operator."""

from typing import Any
import numpy as np
from ._native import RollingWinsorizeOperator as _Native
from ._series import as_float64_series


class RollingWinsorize:
    """Clip each value to lower and upper quantiles of a trailing window.

    ``timeperiod`` defaults to 14 and bounds default to 0.05 and 0.95. The
    first ``timeperiod - 1`` outputs are ``NaN``; lifecycle methods are fluent
    and all arithmetic remains in the native Rust state.
    """

    def __init__(
        self,
        _input: Any,
        timeperiod: int = 14,
        lower: float = 0.05,
        upper: float = 0.95,
    ) -> None:
        """Initialize native state and process the supplied input series.

        Parameters
        ----------
        _input : array-like
            Input history to process in chronological order.
        timeperiod : int, default=14
            Number of observations in the trailing quantile window.
        lower, upper : float, default=0.05, 0.95
            Inclusive lower and upper quantile bounds.

        Returns
        -------
        None
            The constructor initializes native state and returns no value.
        """
        self._state = _Native(timeperiod, lower, upper)
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "RollingWinsorize":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        self._length += 1
        return self

    def extend(self, _input: Any) -> "RollingWinsorize":
        """Append an input history and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned winsorized history as ``np.ndarray``."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest clipped value, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingWinsorize":
        """Reset native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length
