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
        return self._state.value

    @property
    def timeperiod(self) -> int:
        return self._state.timeperiod

    def reset(self) -> "CommodityChannelIndex":
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)



__all__ = ["CommodityChannelIndex"]
