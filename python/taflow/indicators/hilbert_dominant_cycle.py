"""Python adapter for the native Hilbert dominant-cycle estimator."""

from typing import Any

import numpy as np

from .._native import HilbertDominantCycle as _Native
from .._series import as_float64_series


class HilbertDominantCycle:
    """Estimate the dominant market-cycle period using a Hilbert transform.

    The native recursive homodyne discriminator constrains its period estimate
    to 6–50 bars and emits ``NaN`` for the first 50 aligned observations. The
    implementation maps directly to Wickra ``HilbertDominantCycle``.

    Args:
        prices: Chronological price history supplied through ``extend``.
    """

    def __init__(self) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native()

    def append(self, price: float) -> "HilbertDominantCycle":
        """Append one price and return this adapter."""
        self._state.append(float(price))
        return self

    def extend(self, prices: Any) -> "HilbertDominantCycle":
        """Append a chronological price series through native Rust."""
        self._state.extend(as_float64_series(prices))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest estimated period, or ``None`` during warm-up."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned dominant-period history."""
        return self._state.compute()

    def reset(self) -> "HilbertDominantCycle":
        """Reset all recursive phasor state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-price count delegated to native state."""
        return len(self._state)
