"""Causal order-block detection with volatile-bar exclusion and mitigation."""

from typing import Any
import numpy as np
from ._native import OrderBlockOperator as _Native
from ._series import as_float64_series


class OrderBlock:
    """Causal order-block detection with volatile-bar exclusion and mitigation.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `close`, `volume`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        swing_length: int = 50,
        internal_length: int = 5,
        atr_period: int = 200,
        threshold: float = 2.0,
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
        volume : object
            Volume series or the current bar volume.
        swing_length : object
            Number of bars used to confirm a swing.
        internal_length : object
            Internal swing confirmation length in bars.
        atr_period : object
            ATR lookback used for normalization.
        threshold : object
            Detection threshold applied to the input changes.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(swing_length, internal_length, atr_period, threshold)
        (
            self.extend(high, low, close, volume)
            if any(value is not None for value in (high, low, close, volume))
            else None
        )

    def append(self, high: float, low: float, close: float, volume: float) -> "OrderBlock":
        """Append one observation or aligned bar to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.append(high, low, close, volume)
        return self

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "OrderBlock":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        volume : object
            Volume series or the current bar volume.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.extend(
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
        )
        return self

    def compute(
        self,
    ) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
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

    def reset(self) -> "OrderBlock":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
