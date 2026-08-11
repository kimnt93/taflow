"""Causal equal-high/equal-low detection."""

from typing import Any
import numpy as np
from .._adapter_protocol import adapter_length
from .._native import EqualHighsLowsOperator as _Native
from .._series import as_float64_series


class EqualHighsLows:
    """Causal equal-high/equal-low detection.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `close`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        eq_len: int = 3,
        atr_period: int = 200,
        eq_threshold: float = 0.1,
    ) -> None:
        """Initialize this adapter and process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        eq_len : object
            Equal-high/low lookback in bars.
        atr_period : object
            ATR lookback used for normalization.
        eq_threshold : object
            Equality tolerance for level matching.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(eq_len, atr_period, eq_threshold)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "EqualHighsLows":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "EqualHighsLows":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        high_array = as_float64_series(high)
        low_array = as_float64_series(low)
        close_array = as_float64_series(close)
        if not (high_array.shape == low_array.shape == close_array.shape):
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(high_array, low_array, close_array)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return the aligned output history as a NumPy array.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    def reset(self) -> "EqualHighsLows":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return adapter_length(self)
