from typing import Any

import numpy as np

from .._native import HurstChannel as _Native
from .._series import as_float64_series


class HurstChannel:
    """Return rolling range bands around the simple mean of close.

    Output order is ``(upper, middle, lower)`` and width is ``multiplier``
    times the trailing highest-high minus lowest-low. This maps to Wickra
    ``HurstChannel``; Rust owns warm-up and aligned history.

    Args:
        high: Required chronological high-price history.
        low: Required low-price history aligned with ``high``.
        close: Required aligned closing-price history used by the middle mean.
        period: Rolling range and mean period. Defaults to 10.
        multiplier: Fraction of the high-low range added to the mean. Defaults
            to 0.5.

    Raises:
        ValueError: If histories differ in length or configuration is invalid.
    """

    def __init__(
        self,
        high: Any,
        low: Any,
        close: Any,
        period: int = 10,
        multiplier: float = 0.5,
    ) -> None:
        """Initialize native channel state and process aligned histories."""
        self._state = _Native(period, multiplier)
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "HurstChannel":
        """Append one high/low/close bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "HurstChannel":
        """Append aligned high, low, and close histories after validation."""
        arrays = tuple(as_float64_series(item) for item in (high, low, close))
        if len({len(item) for item in arrays}) != 1:
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        return self

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest ``(upper, middle, lower)`` or ``None`` in warm-up."""
        return self._state.value

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned upper, middle, and lower channel histories."""
        return self._state.compute()

    def reset(self) -> "HurstChannel":
        """Clear rolling extrema and mean state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the processed-bar count delegated to native state."""
        return len(self._state)
