"""Native-backed causal rolling fractal-dimension adapter."""

from typing import Any

import numpy as np

from .._native import FractalDimensionOperator as _Native
from .._series import as_float64_series


class FractalDimension:
    """Estimate rolling fractal dimension as ``2 - Hurst exponent``.

    Rust owns the two-chunk rescaled-range calculation and persistent state.
    ``prices`` is a required chronological series; an empty array creates a
    fresh stream. ``timeperiod`` defaults to 20 and must be at least four.
    Aligned history contains ``NaN`` until one full period is available. The
    independent formula oracle is the registered NumPy rescaled-range model.

    Args:
        prices: Required chronological price history.
        timeperiod: Trailing rescaled-range window. Defaults to 20.

    Raises:
        ValueError: If ``timeperiod`` is less than four.
    """

    def __init__(self, prices: Any, timeperiod: int = 20) -> None:
        """Initialize native state and process the supplied price history."""
        self._state = _Native(timeperiod)
        self.extend(prices)

    def append(self, price: float) -> "FractalDimension":
        """Append one price and return this adapter for method chaining."""
        self._state.append(float(price))
        return self

    def extend(self, prices: Any) -> "FractalDimension":
        """Append a chronological price series and return this adapter."""
        self._state.extend(as_float64_series(prices))
        return self

    def compute(self) -> np.ndarray:
        """Return aligned fractal-dimension history as a NumPy array."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest dimension, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "FractalDimension":
        """Reset native state and return this adapter for method chaining."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-price count delegated to native state."""
        return len(self._state)


__all__ = ["FractalDimension"]
