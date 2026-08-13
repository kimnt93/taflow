"""Python adapter for the native non-repainting ZigZag detector."""

from typing import Any

import numpy as np

from .._native import ZigZag as _Native
from .._series import as_float64_series


class ZigZag:
    """Confirm high and low swings after a percentage reversal.

    The first bar seeds an uptrend at its high. A swing high is confirmed after
    ``low`` falls by ``threshold`` from the running high; a swing low is
    confirmed after ``high`` rises by that fraction from the running low.
    Output order is ``(swing, direction)``, where direction is ``1.0`` for a
    high and ``-1.0`` for a low. Non-confirmation rows are ``NaN``. This maps to
    Wickra ``ZigZag``.

    Args:
        high: Required chronological high-price history.
        low: Required chronological low-price history aligned with ``high``.
        threshold: Fractional reversal in ``(0, 1)``. Defaults to 0.05.

    Raises:
        ValueError: If inputs differ in length or ``threshold`` is invalid.
    """

    def __init__(self, threshold: float = 0.05) -> None:
        """Initialize an empty configured native state.
        """
        self._state = _Native(threshold)

    def append(self, high: float, low: float) -> "ZigZag":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        return self

    def extend(self, high: Any, low: Any) -> "ZigZag":
        """Append aligned high and low histories after validating their lengths."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the swing confirmed by the latest bar, if one was emitted."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned ``(swing, direction)`` histories."""
        return self._state.compute()

    def reset(self) -> "ZigZag":
        """Clear the running extreme and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
