"""Python adapter for the native Zweig Breadth Thrust series."""

from typing import Any
import numpy as np
from .._native import BreadthThrust as _Native
from .._series import as_float64_series


class BreadthThrust:
    """Smooth the share of participating issues that are advancing.

    Native Rust averages ``advancers / max(advancers + decliners, 1)`` over
    ``period`` ticks. Warm-up rows are ``NaN``. This maps to Wickra
    ``BreadthThrust``.

    Args:
        advancers: Number of advancing issues at each tick.
        decliners: Number of declining issues at each aligned tick.
        period: Simple moving-average period. Defaults to 10.

    Raises:
        ValueError: If histories differ in length or ``period`` is zero.
    """

    def __init__(self, advancers: Any, decliners: Any, period: int = 10) -> None:
        """Initialize native rolling state and process aligned histories."""
        self._state = _Native(period)
        self.extend(advancers, decliners)

    def append(self, advancers: float, decliners: float) -> "BreadthThrust":
        """Append one breadth tick and return this adapter."""
        self._state.append(float(advancers), float(decliners))
        return self

    def extend(self, advancers: Any, decliners: Any) -> "BreadthThrust":
        """Append aligned breadth histories after length validation."""
        arrays = as_float64_series(advancers), as_float64_series(decliners)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("advancers and decliners must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> float | None:
        """Return the latest thrust value, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return aligned Breadth Thrust history with warm-up as ``NaN``."""
        return self._state.compute()

    def reset(self) -> "BreadthThrust":
        """Clear the rolling window and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-tick count delegated to native state."""
        return len(self._state)
