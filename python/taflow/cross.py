from typing import Any
import numpy as np
from ._native import CrossOperator as _Native
from ._series import as_float64_series


class Cross:
    """Detect either a causal upward or downward crossing.

    ``left`` and ``right`` are required aligned numeric histories. Each output
    is 1.0 on a crossing and 0.0 otherwise; the first bar is 0.0 because no
    prior pair exists. ``append``, ``extend``, and ``reset`` are fluent,
    ``value`` returns the latest scalar or ``None``, and ``compute`` returns a
    NumPy array.
    """

    def __init__(
        self,
        left: Any,
        right: Any,
    ) -> None:
        """Create the native state and replay aligned input histories."""
        self._state = _Native()
        self.extend(left, right)

    def append(self, left: float, right: float) -> "Cross":
        """Append one pair and return this adapter."""
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any) -> "Cross":
        """Append aligned left/right histories and return this adapter."""
        arrays = tuple(as_float64_series(series) for series in (left, right))
        if arrays[0].shape != arrays[1].shape:
            raise ValueError("left and right must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned 0.0/1.0 crossing history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest crossing flag or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "Cross":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed pairs."""
        return len(self._state.compute())
