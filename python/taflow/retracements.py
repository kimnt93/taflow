"""Native-backed causal swing retracement adapter."""

from typing import Any

import numpy as np

from ._native import RetracementsOperator as _Native
from ._series import as_float64_series


class Retracements:
    """Track swing-leg retracement, extension, and direction values.

    ``high``, ``low``, and ``close`` are required equal-length chronological
    histories and may all be empty for a fresh stream. ``swing_length``
    defaults to 5. Rust owns swing confirmation, causal alignment, warm-up,
    and the three output series; ``compute`` returns ``(retracement,
    extension, direction)`` arrays. ``value`` exposes the latest tuple or
    ``None`` and lifecycle mutators return ``self``.
    """

    def __init__(
        self, high: Any, low: Any, close: Any, swing_length: int = 5
    ) -> None:
        self._state = _Native(int(swing_length))
        self._length = 0
        self.extend(high, low, close)

    def append(self, high: float, low: float, close: float) -> "Retracements":
        """Append one OHLC bar and return this adapter."""
        self._state.append(float(high), float(low), float(close))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any, close: Any) -> "Retracements":
        """Append equal-length high, low, and close histories."""
        arrays = tuple(as_float64_series(series) for series in (high, low, close))
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, and close must have equal lengths")
        self._state.extend(*arrays)
        self._length += len(arrays[0])
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned retracement, extension, and direction arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest retracement tuple, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Retracements":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length


__all__ = ["Retracements"]
