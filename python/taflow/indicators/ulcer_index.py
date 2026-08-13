"""Native-backed Ulcer Index adapter."""

from typing import Any

import numpy as np

from .._native import UlcerIndexOperator as _Native
from .._series import as_float64_series


class UlcerIndex:
    """Compute the trailing root-mean-square percentage drawdown.

    ``_input`` is the required chronological price series and may be empty for
    a fresh stream. ``timeperiod`` defaults to 14. Rust owns running maxima,
    drawdown squares, NaN warm-up, and aligned history. ``compute`` returns one
    float array, ``value`` is the latest index or ``None`` during warm-up, and
    lifecycle mutators return ``self``. The oracle is pandas rolling
    ``cummax``/root-mean-square arithmetic.
    """

    def __init__(self, timeperiod: int = 14) -> None:
        self._state = _Native(int(timeperiod))

    def append(self, _input: float) -> "UlcerIndex":
        """Append one price and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "UlcerIndex":
        """Append a chronological price series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned Ulcer Index history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest index, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "UlcerIndex":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed prices."""
        return len(self._state)


__all__ = ["UlcerIndex"]
