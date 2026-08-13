"""Python adapter for the native Absolute Breadth Index."""

from typing import Any
import numpy as np
from .._native import AbsoluteBreadthIndex as _Native
from .._series import as_float64_series


class AbsoluteBreadthIndex:
    """Return the magnitude of net advancing issues for each market tick.

    The result is ``abs(advancers - decliners)`` and is available from the first
    tick. Inputs are pre-aggregated cross-sectional counts. This maps to Wickra
    ``AbsoluteBreadthIndex``.

    Args:
        advancers: Number of advancing issues at each tick.
        decliners: Number of declining issues at each aligned tick.

    Raises:
        ValueError: If the input histories have different lengths.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(self, advancers: float, decliners: float) -> "AbsoluteBreadthIndex":
        """Append one breadth tick and return this adapter."""
        self._state.append(float(advancers), float(decliners))
        return self

    def extend(self, advancers: Any, decliners: Any) -> "AbsoluteBreadthIndex":
        """Append aligned breadth histories after validating their lengths."""
        arrays = as_float64_series(advancers), as_float64_series(decliners)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("advancers and decliners must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest breadth magnitude, or ``None`` before input."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned Absolute Breadth Index history."""
        return self._state.compute()

    def reset(self) -> "AbsoluteBreadthIndex":
        """Reset native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
