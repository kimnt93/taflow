"""Average daily dollar value traded."""

from typing import Any
import numpy as np
from ._native import AverageDailyDollarValueOperator as _Native
from ._series import as_float64_series


class AverageDailyDollarValue:
    """Rolling simple average of aligned ``close * volume`` values."""

    def __init__(self, close: Any, volume: Any, timeperiod: int = 20) -> None:
        """Create native state and replay required aligned histories."""
        self._state = _Native(timeperiod)
        self._length = 0
        self.extend(close, volume)

    def append(self, close: float, volume: float) -> "AverageDailyDollarValue":
        """Append one bar and return this adapter."""
        self._state.append(float(close), float(volume))
        self._length += 1
        return self

    def extend(self, close: Any, volume: Any) -> "AverageDailyDollarValue":
        """Append aligned close and volume histories and return this adapter."""
        close_array = as_float64_series(close)
        volume_array = as_float64_series(volume)
        if close_array.shape != volume_array.shape:
            raise ValueError("close and volume must have equal lengths")
        self._state.extend(close_array, volume_array)
        self._length += len(close_array)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rolling average history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest result or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "AverageDailyDollarValue":
        """Reset native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length
