"""Native-backed causal rolling-Sortino adapter."""

from typing import Any

import numpy as np

from .._native import RollingSortino as _Native
from .._adapter_protocol import adapter_length
from .._series import as_float64_series


class RollingSortino:
    """Compute an unannualized rolling Sortino ratio.

    ``_input`` is the required chronological series and may be empty for a
    fresh stream. ``timeperiod`` defaults to 14; Rust emits NaN until the
    trailing window is full and uses downside deviation for the denominator.
    ``compute`` returns one aligned float array, ``value`` is the latest scalar
    or ``None`` during warm-up, and lifecycle mutators return ``self``. The
    oracle is pandas rolling downside-deviation arithmetic.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))

    def append(self, _input: float) -> "RollingSortino":
        """Append one observation and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "RollingSortino":
        """Append a chronological observation series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned rolling-Sortino history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest ratio, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "RollingSortino":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed observations."""
        return adapter_length(self)


__all__ = ["RollingSortino"]
