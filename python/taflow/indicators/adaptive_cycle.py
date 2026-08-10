"""Python adapter for the native adaptive-cycle state."""

from typing import Any

import numpy as np

from .._native import AdaptiveCycle as _Native
from .._series import as_float64_series


class AdaptiveCycle:
    """Measure price change over half of the Hilbert dominant-cycle period.

    Rust estimates the dominant cycle, rounds half that period to a lag in the
    inclusive range 3–25, and subtracts the lagged price. The first 50 aligned
    outputs are ``NaN`` while the Hilbert state settles. This definition and
    canonical name map to Wickra ``AdaptiveCycle``.

    Args:
        prices: Required chronological price history. An empty series creates a
            fresh state for streaming updates.
    """

    def __init__(self, prices: Any) -> None:
        """Initialize the native state and process the supplied price history."""
        self._state = _Native()
        self.extend(prices)

    def append(self, price: float) -> "AdaptiveCycle":
        """Append one price and return this adapter for method chaining."""
        self._state.append(float(price))
        return self

    def extend(self, prices: Any) -> "AdaptiveCycle":
        """Append a chronological price series through the native bulk path."""
        self._state.extend(as_float64_series(prices))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest cycle change, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned cycle-change history with warm-up as ``NaN``."""
        return self._state.compute()

    def reset(self) -> "AdaptiveCycle":
        """Clear native history and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-price count delegated to native state."""
        return len(self._state)
