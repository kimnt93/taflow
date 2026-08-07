"""Persistent Money Flow Index indicator."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import MoneyFlowIndex as _NativeMoneyFlowIndex
from ._series import as_float64_series


class MoneyFlowIndex:
    """Compute MFI history once, then append HLCV bars in O(1)."""

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        volume: Any | None = None,
        timeperiod: int = 14,
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        volume : object
            Input series, scalar parameter, or configuration value for this operation.
        timeperiod : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
            The updated adapter, native value, aligned output array, or execution node.
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
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        volume : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
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
            Input series, scalar parameter, or configuration value for this operation.
        low : object
            Input series, scalar parameter, or configuration value for this operation.
        close : object
            Input series, scalar parameter, or configuration value for this operation.
        volume : object
            Input series, scalar parameter, or configuration value for this operation.

        Returns
        -------
        object
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
        object
            The updated adapter, native value, aligned output array, or execution node.
        """
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest computed value, or None during warm-up.

        Returns
        -------
        object
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
        object
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
