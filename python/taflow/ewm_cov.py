"""Persistent exponentially weighted covariance."""

from typing import Any
import numpy as np
from ._native import EwmCovOperator as _Native
from ._series import as_float64_series


class ExponentiallyWeightedCovariance:
    """Persistent exponentially weighted covariance.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `left`, `right`. Warm-up positions are represented by `NaN` in history."""

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
        self._state = _Native(timeperiod)
        self.extend(left, right) if left is not None or right is not None else None

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
        self._state.append(left, right)
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
        self._state.extend(as_float64_series(left), as_float64_series(right))
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
    def value(self) -> object:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "ExponentiallyWeightedCovariance":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
