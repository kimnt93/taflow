"""Persistent Money Flow Index indicator."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import MoneyFlowIndex as _NativeMoneyFlowIndex
from ._series import as_float64_series


class MoneyFlowIndex:
    """Compute MFI history once, then append HLCV bars in O(1)

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    MoneyFlowIndex
        A persistent native-backed indicator adapter.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        volume: Any,
        timeperiod: int = 14,
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
        timeperiod : object
            Trailing window length in bars.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _NativeMoneyFlowIndex(timeperiod)
        if (
            high is not None
            or low is not None
            or close is not None
            or volume is not None
        ):
            self.extend(high, low, close, volume)

    def append(
        self, high: float, low: float, close: float, volume: float
    ) -> "MoneyFlowIndex":
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
        self._state.append(float(high), float(low), float(close), float(volume))
        return self

    def extend(
        self,
        high: Any,
        low: Any | None = None,
        close: Any | None = None,
        volume: Any | None = None,
    ) -> "MoneyFlowIndex":
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
        if low is None or close is None or volume is None:
            raise ValueError("high, low, close, and volume must be provided together")
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
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        float, tuple, or None
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.value

    @property
    def timeperiod(self) -> int:
        """Execute the timeperiod operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.timeperiod

    def reset(self) -> "MoneyFlowIndex":
        """Execute the reset operation through the native Rust implementation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Execute the __len__ operation through the native Rust implementation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return len(self._state)


__all__ = ["MoneyFlowIndex"]
