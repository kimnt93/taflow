"""Persistent exponentially weighted covariance."""

from typing import Any

import numpy as np

from .._native import ExponentiallyWeightedCovariance as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class ExponentiallyWeightedCovariance:
    """Compute causal exponentially weighted population covariance.

    ``left`` and ``right`` are required equal-length chronological series and
    may both be empty for a fresh stream. ``timeperiod`` defaults to 14 and is
    interpreted as a pandas EWM span. Rust owns the paired recurrence and
    aligned history; ``compute`` returns one float array and ``value`` is the
    latest covariance or ``None`` before the first pair. Lifecycle mutators
    return ``self`` and reject length mismatches before mutation. The oracle is
    pandas ``ExponentialMovingWindow.cov(bias=True)``.
    """

    def __init__(
        self,
        timeperiod: int = 14,
    ) -> None:
        """Initialize an empty configured native state.

        Parameters
        ----------
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(int(timeperiod))

    def append(self, left: float, right: float) -> "ExponentiallyWeightedCovariance":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        left : object
            Left-hand aligned input series or scalar value.
        right : object
            Right-hand aligned input series or scalar value.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any) -> "ExponentiallyWeightedCovariance":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        left : object
            Left-hand aligned input series or scalar value.
        right : object
            Right-hand aligned input series or scalar value.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        left_values = as_float64_series(left)
        right_values = as_float64_series(right)
        if len(left_values) != len(right_values):
            raise ValueError("left and right input series must have equal length")
        self._state.extend(left_values, right_values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def __len__(self) -> int:
        """Return the number of paired observations consumed by this state."""
        return adapter_length(self)

    def reset(self) -> "ExponentiallyWeightedCovariance":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self


__all__ = ["ExponentiallyWeightedCovariance"]
