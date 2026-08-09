"""Native-backed Donchian-channel adapter."""

from typing import Any

import numpy as np

from ._native import DonchianOperator as _Native
from ._series import as_float64_series


class Donchian:
    """Compute causal trailing high, low, and midpoint channels.

    ``high`` and ``low`` are required equal-length chronological histories and
    may both be empty for a fresh stream. ``timeperiod`` defaults to 20. The
    first ``timeperiod - 1`` outputs are NaN; ``compute`` returns
    ``(upper, lower, middle)`` arrays. Lifecycle mutators return ``self`` and
    ``value`` exposes the latest tuple or ``None``. The oracle is pandas
    rolling max/min.
    """

    def __init__(self, high: Any, low: Any, timeperiod: int = 20) -> None:
        self._state = _Native(int(timeperiod))
        self._length = 0
        self.extend(high, low)

    def append(self, high: float, low: float) -> "Donchian":
        """Append one high/low bar and return this adapter."""
        self._state.append(float(high), float(low))
        self._length += 1
        return self

    def extend(self, high: Any, low: Any) -> "Donchian":
        """Append equal-length high and low histories."""
        arrays = as_float64_series(high), as_float64_series(low)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("high and low must have equal lengths")
        self._state.extend(*arrays)
        self._length += len(arrays[0])
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned ``(upper, lower, middle)`` arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest channel tuple, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Donchian":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return self._length


__all__ = ["Donchian"]
