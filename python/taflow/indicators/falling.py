"""Native-backed causal falling-signal adapter."""

from typing import Any

import numpy as np

from .._native import FallingOperator as _Native
from .._series import as_float64_series


class Falling:
    """Emit one when the latest value falls under the trailing comparison.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 1 and must be positive. Rust owns
    the causal comparison and warm-up; ``compute`` returns one aligned float
    array, ``value`` is the latest flag or ``None`` during warm-up, and
    lifecycle mutators return ``self``. The oracle is pandas rolling comparison
    arithmetic.
    """

    def __init__(self, _input: Any, timeperiod: int = 1) -> None:
        self._state = _Native(int(timeperiod))
        self.extend(_input)

    def append(self, _input: float) -> "Falling":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "Falling":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned falling flags."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest flag, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "Falling":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return len(self._state)


__all__ = ["Falling"]
