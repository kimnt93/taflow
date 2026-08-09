"""Persistent Chaikin Money Flow."""

from typing import Any

import numpy as np

from ._native import ChaikinMoneyFlowOperator as _Native
from ._series import as_float64_series


class ChaikinMoneyFlow:
    """Persistent Chaikin Money Flow.

    This public class owns a persistent native Rust state; Python performs container conversion only. `append`, `extend`, and `reset` are fluent, `value` exposes the latest result, and `compute` returns aligned history. Required input histories: `high`, `low`, `close`, `volume`. Warm-up positions are represented by `NaN` in history."""

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        period: object = 20,
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
        period : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _Native(period)
        if any(value is not None for value in (high, low, close, volume)):
            self.extend(high, low, close, volume)

    def append(self, high: float, low: float, close: float, volume: float) -> "ChaikinMoneyFlow":
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

    def extend(self, high: Any, low: Any, close: Any, volume: Any) -> "ChaikinMoneyFlow":
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

    def reset(self) -> "ChaikinMoneyFlow":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self
