"""Persistent causal rolling covariance operator."""

from typing import Any
import numpy as np
from ._native import RollingCovarianceOperator as _Native
from ._series import as_float64_series


class RollingCovariance:
    """Population covariance over two required aligned trailing series.

    ``timeperiod`` defaults to 14. The first ``timeperiod - 1`` aligned
    outputs are ``NaN``; ``append``, ``extend``, and ``reset`` mutate and
    return this adapter, while ``value`` exposes the latest scalar result.
    TAFlow's ``RollingCovariance`` is compared with pandas rolling covariance
    using ``ddof=0``.
    """

    def __init__(
        self,
        left: Any,
        right: Any,
        timeperiod: int = 14,
    ) -> None:
        """Initialize native state and process the supplied aligned series.

        Parameters
        ----------
        left, right : array-like
            Equal-length aligned input histories.
        timeperiod : int, default=14
            Number of observations in the trailing population window.

        Returns
        -------
        None
            The constructor initializes the native state and returns no value.
        """
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(left, right)

    def append(self, left: float, right: float) -> "RollingCovariance":
        """Append one ``left``/``right`` pair and return this adapter."""
        self._state.append(float(left), float(right))
        self._length += 1
        return self

    def extend(self, left: Any, right: Any) -> "RollingCovariance":
        """Append equal-length aligned ``left`` and ``right`` histories."""
        left_array = as_float64_series(left)
        right_array = as_float64_series(right)
        if len(left_array) != len(right_array):
            raise ValueError("left and right must have equal lengths")
        self._state.extend(left_array, right_array)
        self._length += len(left_array)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned population-covariance history as ``np.ndarray``."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest covariance, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingCovariance":
        """Reset native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length
