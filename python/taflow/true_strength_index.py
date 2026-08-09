"""Native-backed True Strength Index adapter."""

from typing import Any

import numpy as np

from ._native import TrueStrengthIndexOperator as _Native
from ._adapter_protocol import adapter_length
from ._series import as_float64_series


class TrueStrengthIndex:
    """Compute the double-smoothed momentum True Strength Index.

    ``_input`` is the required chronological close series and may be empty for
    a fresh stream. ``fast`` defaults to 13 and ``slow`` to 25. Rust owns price
    differences, absolute differences, double EMA smoothing, warm-up, and
    aligned output. ``compute`` returns one float array, ``value`` is the latest
    scalar or ``None`` during warm-up, and lifecycle mutators return ``self``.
    The oracle is pandas-ta-classic ``tsi``; TAFlow records its seeding variant
    where applicable.
    """

    def __init__(self, _input: Any, fast: int = 13, slow: int = 25) -> None:
        self._state = _Native(int(fast), int(slow))
        self.extend(_input)

    def append(self, _input: float) -> "TrueStrengthIndex":
        """Append one close and return this adapter."""
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any) -> "TrueStrengthIndex":
        """Append a chronological close series and return this adapter."""
        values = as_float64_series(_input)
        self._state.extend(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned True Strength Index history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest index, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "TrueStrengthIndex":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed closes."""
        return adapter_length(self)


__all__ = ["TrueStrengthIndex"]
