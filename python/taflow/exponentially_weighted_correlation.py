"""Persistent exponentially weighted correlation."""

from typing import Any

import numpy as np

from ._native import ExponentiallyWeightedCorrelationOperator as _Native
from ._series import as_float64_series


class ExponentiallyWeightedCorrelation:
    """Compute causal exponentially weighted correlation.

    ``left`` and ``right`` are required equal-length chronological series and
    may both be empty for a fresh stream. ``timeperiod`` defaults to 14 and is
    interpreted as a pandas EWM span. Rust owns paired means, variances,
    covariance, zero-variance handling, and aligned history. ``compute``
    returns one float array; ``value`` is the latest correlation or ``None``
    before the first pair. Lifecycle mutators return ``self`` and reject length
    mismatches before mutation. The oracle is pandas
    ``ExponentialMovingWindow.corr`` with its initial undefined value mapped to
    zero by the TAFlow contract.
    """

    def __init__(
        self,
        left: Any,
        right: Any,
        timeperiod: int = 14,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        timeperiod : object
            Trailing window length in bars.
        left : object
            Left-hand aligned input series or scalar value.
        right : object
            Right-hand aligned input series or scalar value.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(int(timeperiod))
        self._length = 0
        self.extend(left, right)

    def append(self, left: float, right: float) -> "ExponentiallyWeightedCorrelation":
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
        self._length += 1
        return self

    def extend(self, left: Any, right: Any) -> "ExponentiallyWeightedCorrelation":
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
        self._length += len(left_values)
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
        return self._length

    def reset(self) -> "ExponentiallyWeightedCorrelation":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self
