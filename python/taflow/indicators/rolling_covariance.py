"""Native-backed causal rolling covariance adapter."""

from typing import Any

import numpy as np

from .._native import RollingCovariance as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class RollingCovariance:
    """Compute population covariance over two aligned trailing series.

    ``left`` and ``right`` are required equal-length chronological series and
    may both be empty for a fresh stream. ``timeperiod`` defaults to 14; the
    first ``timeperiod - 1`` outputs are NaN. ``compute`` returns one aligned
    float array, ``value`` is the latest scalar or ``None`` during warm-up,
    and lifecycle mutators return ``self``. The oracle is pandas rolling
    covariance with ``ddof=0``.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        """Initialize an empty configured native state.

        Parameters
        ----------
        timeperiod : int, default=14
            Number of observations in the trailing population window.

        Returns
        -------
        None
            The constructor initializes the native state and returns no value.
        """
        self._state = _Native(int(timeperiod))

    def append(self, left: float, right: float) -> "RollingCovariance":
        """Append one ``left``/``right`` pair and return this adapter."""
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any) -> "RollingCovariance":
        """Append equal-length aligned histories and return this adapter."""
        left_array = as_float64_series(left)
        right_array = as_float64_series(right)
        if len(left_array) != len(right_array):
            raise ValueError("left and right must have equal lengths")
        self._state.extend(left_array, right_array)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned population-covariance history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest covariance, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingCovariance":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return adapter_length(self)


__all__ = ["RollingCovariance"]
