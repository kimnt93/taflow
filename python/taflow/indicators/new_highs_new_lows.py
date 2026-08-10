"""Python adapter for native New Highs minus New Lows breadth."""

from typing import Any
import numpy as np
from .._native import NewHighsNewLows as _Native
from .._series import as_float64_series


class NewHighsNewLows:
    """Return ``new_highs - new_lows`` for each cross-sectional tick.

    Inputs are pre-aggregated constituent counts. The definition maps directly
    to Wickra ``NewHighsNewLows`` and emits from the first tick.

    Args:
        new_highs: Number of constituents making a new high.
        new_lows: Number making a new low at each aligned tick.

    Raises:
        ValueError: If the two histories have different lengths.
    """

    def __init__(self, new_highs: Any, new_lows: Any) -> None:
        """Initialize native state and process aligned count histories."""
        self._state = _Native()
        self.extend(new_highs, new_lows)

    def append(self, new_highs: float, new_lows: float) -> "NewHighsNewLows":
        """Append one aggregate-extremes tick and return this adapter."""
        self._state.append(float(new_highs), float(new_lows))
        return self

    def extend(self, new_highs: Any, new_lows: Any) -> "NewHighsNewLows":
        """Append aligned count histories after validating their lengths."""
        arrays = as_float64_series(new_highs), as_float64_series(new_lows)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("new-high and new-low counts must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest net count, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned net new-extremes history."""
        return self._state.compute()

    def reset(self) -> "NewHighsNewLows":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
