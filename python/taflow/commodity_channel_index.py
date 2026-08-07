"""Persistent Commodity Channel Index indicator."""

from __future__ import annotations

from typing import Any

import numpy as np

from ._native import CommodityChannelIndex as _NativeCommodityChannelIndex
from ._series import as_float64_series


class CommodityChannelIndex:
    """Compute CCI history once, then continue it with new HLC bars."""

    def __init__(
        self,
        high: Any | None = None,
        low: Any | None = None,
        close: Any | None = None,
        timeperiod: int = 14,
        *,
        high_column: str = "high",
        low_column: str = "low",
        close_column: str = "close",
    ) -> None:
        """Initialize this adapter and optionally process the supplied input series.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        timeperiod : object
            Trailing window length in bars.
        high_column : object
            Values or parameters consumed by this operation.
        low_column : object
            Values or parameters consumed by this operation.
        close_column : object
            Values or parameters consumed by this operation.

        Returns
        -------
        None
            The constructor initializes the adapter and returns no value.
        """
        self._state = _NativeCommodityChannelIndex(timeperiod)
        if high is not None or low is not None or close is not None:
            self.extend(
                high,
                low,
                close,
                high_column=high_column,
                low_column=low_column,
                close_column=close_column,
            )

    def append(self, high: float, low: float, close: float) -> "CommodityChannelIndex":
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

    def extend(
        self,
        high: Any,
        low: Any | None = None,
        close: Any | None = None,
        *,
        high_column: str = "high",
        low_column: str = "low",
        close_column: str = "close",
    ) -> "CommodityChannelIndex":
        """Append aligned input series to the native Rust state.

        Parameters
        ----------
        high : object
            High-price series or the current bar high.
        low : object
            Low-price series or the current bar low.
        close : object
            Close-price series or the current bar close.
        high_column : object
            Values or parameters consumed by this operation.
        low_column : object
            Values or parameters consumed by this operation.
        close_column : object
            Values or parameters consumed by this operation.

        Returns
        -------
        Self
            The updated adapter, native value, aligned output array, or execution node.
        """
        if low is None and close is None and hasattr(high, "columns"):
            frame = high
            high = as_float64_series(frame, column=high_column)
            low = as_float64_series(frame, column=low_column)
            close = as_float64_series(frame, column=close_column)
        elif low is None or close is None:
            raise ValueError("high, low, and close must be provided together")
        else:
            high = as_float64_series(high)
            low = as_float64_series(low)
            close = as_float64_series(close)

        self._state.extend(high, low, close)
        return self

    def compute(self) -> np.ndarray:
        """Return every aligned CCI result accumulated by this object."""

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

    def reset(self) -> "CommodityChannelIndex":
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


__all__ = ["CommodityChannelIndex"]
