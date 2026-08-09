"""Native-backed entry/exit signal adapter."""

from typing import Any

import numpy as np

from ._native import EntryExitOperator as _Native
from ._series import as_bool_series


class EntryExit:
    """Track entry and exit events as a causal position signal.

    ``entry`` and ``exit`` are required equal-length chronological boolean
    histories and may both be empty for a fresh stream. An entry sets +1, an
    exit sets -1, and an inactive or simultaneous pair holds the previous
    position. ``compute`` returns one aligned float array, ``value`` is the
    latest position or ``None`` for an empty stream, and lifecycle mutators
    return ``self``.
    """

    def __init__(self, entry: Any, exit: Any) -> None:
        self._state = _Native()
        self._length = 0
        self.extend(entry, exit)

    def append(self, entry: bool, exit: bool) -> "EntryExit":
        """Append one entry/exit pair and return this adapter."""
        self._state.append(bool(entry), bool(exit))
        self._length += 1
        return self

    def extend(self, entry: Any, exit: Any) -> "EntryExit":
        """Append equal-length boolean event histories."""
        arrays = as_bool_series(entry), as_bool_series(exit)
        if len(arrays[0]) != len(arrays[1]):
            raise ValueError("entry and exit must have equal lengths")
        self._state.extend(*arrays)
        self._length += len(arrays[0])
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned causal position history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest position, or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "EntryExit":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        self._length = 0
        return self

    def __len__(self) -> int:
        """Return the number of processed event pairs."""
        return self._length


__all__ = ["EntryExit"]
