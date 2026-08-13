"""Public adapter for the native Ehlers Decycler."""

from typing import Any
import numpy as np
from .._native import Decycler as _Native
from .._series import as_float64_series


class Decycler:
    """Remove the two-pole high-pass component from a chronological series.

    The output is ``input - high_pass(input)`` with Ehlers coefficients and
    first-bar initialization. It maps to Wickra ``Decycler``.

    Args:
        values: Chronological price or signal history.
        period: Positive high-pass critical period, default 20.

    Raises:
        ValueError: If ``period`` is zero.
    """

    def __init__(self, period: int = 20) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(period)

    def append(self, value: float) -> "Decycler":
        """Append one signal value and return this adapter."""
        self._state.append(float(value))
        return self

    def extend(self, values: Any) -> "Decycler":
        """Append one converted float64 history and return this adapter."""
        self._state.extend(as_float64_series(values))
        return self

    @property
    def value(self) -> float | None:
        """Return the latest trend component, or ``None`` when empty."""
        return self._state.value

    def compute(self) -> np.ndarray:
        """Return the aligned native trend history."""
        return self._state.compute()

    def reset(self) -> "Decycler":
        """Reset native filter history and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
