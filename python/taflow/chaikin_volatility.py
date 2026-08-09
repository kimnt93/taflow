"""Persistent Chaikin Volatility."""

from typing import Any
import numpy as np
from ._native import ChaikinVolatilityOperator as _Native
from ._series import as_float64_series


class ChaikinVolatility:
    """Persistent Chaikin Volatility.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        timeperiod: int = 10,
        roc_period: int = 10,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        timeperiod : object
            Trailing window length in bars.
        roc_period : object
            Rate-of-change lookback period.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(timeperiod, roc_period)
        self._length = 0
        self.extend(high, low)

    def append(self, high: float, low: float) -> "ChaikinVolatility":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(high), float(low))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any) -> "ChaikinVolatility":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        high_values = as_float64_series(high)
        low_values = as_float64_series(low)
        if len(high_values) != len(low_values):
            raise ValueError("high and low input series must have equal length")
        self._state.extend(high_values, low_values)
        self._length += len(high_values)
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

    def reset(self) -> "ChaikinVolatility":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        self._length = 0
        return self
