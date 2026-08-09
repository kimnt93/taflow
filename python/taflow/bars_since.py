"""Native-backed bars-since signal adapter."""

from typing import Any

import numpy as np

from ._native import BarsSinceOperator as _Native
from ._series import as_bool_series


class BarsSince:
    """Count bars since the most recent true condition.

    ``condition`` is the required chronological boolean series and may be
    empty for a fresh stream. Rust emits the causal count with NaN before the
    first true condition; ``compute`` returns one aligned float array and
    ``value`` is the latest scalar or ``None`` when no output exists. Lifecycle
    mutators return ``self``. No independent external oracle is available for
    this stateful signal definition.
    """

    def __init__(self, condition: Any) -> None:
        self._state = _Native()
        self._length = 0
        self.extend(condition)

    def append(self, condition: bool) -> "BarsSince":
        """Append one condition and return this adapter."""
        self._state.append(bool(condition))
        self._length += 1
        return self

    def extend(self, condition: Any) -> "BarsSince":
        """Append a chronological boolean condition series."""
        values = as_bool_series(condition)
        self._state.extend(values)
        self._length += len(values)
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned bars-since counts."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest count, or ``None`` before the first true value."""
        return self._state.value

    def reset(self) -> "BarsSince":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed conditions."""
        return self._length


__all__ = ["BarsSince"]
