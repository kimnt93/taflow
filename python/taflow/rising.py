"""Native-backed causal rising-signal adapter."""

from typing import Any

import numpy as np

from ._native import RisingOperator as _Native
from ._series import as_float64_series


class Rising:
    """Emit one when the latest value rises over the trailing comparison.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 1 and must be positive. Rust owns
    the causal comparison and warm-up; ``compute`` returns one aligned float
    array, ``value`` is the latest flag or ``None`` during warm-up, and
    lifecycle mutators return ``self``. The oracle is pandas rolling comparison
    arithmetic.
    """

    def __init__(self, _input: Any, timeperiod: int = 1) -> None:
        self._state = _Native(int(timeperiod))
        self._length = 0
        self.extend(_input)

    def append(self, _input: float) -> "Rising":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        self._length += 1
        return self

    def extend(self, _input: Any) -> "Rising":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rising flags."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest flag, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Rising":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return self._length


__all__ = ["Rising"]
