"""Causal equal-high/equal-low detection."""

from typing import Any
import numpy as np
from ._native import EqualHighsLowsOperator as _Native
from ._series import as_float64_series


class EqualHighsLows:
    """Stateful EqualHighsLows indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        eq_len: int = 3,
        atr_period: int = 200,
        eq_threshold: float = 0.1,
    ):
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        eq_len : object
            Input series, scalar parameter, or configuration value for this operation.
        atr_period : object
            Input series, scalar parameter, or configuration value for this operation.
        eq_threshold : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state = _Native(eq_len, atr_period, eq_threshold)
        (
            self.extend(high, low, close)
            if any(value is not None for value in (high, low, close))
            else None
        )

    def append(self, high: float, low: float, close: float):
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, close)
        return self

    def extend(self, high: Any, low: Any, close: Any):
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            as_float64_series(high), as_float64_series(low), as_float64_series(close)
        )
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self):
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self):
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
