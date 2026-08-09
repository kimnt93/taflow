"""Persistent entry/exit signal adapter."""

from typing import Any

import numpy as np

from ._native import EntryExitOperator
from ._series import as_bool_series


class EntryExit:
    """Track entry and exit events as a causal position signal.

    ``entry`` and ``exit`` are required aligned boolean histories; empty arrays
    create a fresh stream. An entry sets position +1, an exit sets -1, and a
    simultaneous or inactive pair holds the previous position. Lifecycle
    methods are fluent, ``value`` returns the latest scalar or ``None``, and
    ``compute`` returns a NumPy history.
    """

    def __init__(
        self,
        entry: Any,
        exit: Any,
    ) -> None:
        """Create the native state and replay aligned event histories."""
        self._state = EntryExitOperator()
        self.extend(entry, exit)

    def append(self, entry: bool, exit: bool) -> "EntryExit":
        """Append one entry/exit pair and return this adapter."""
        self._state.append(bool(entry), bool(exit))
        return self

    def extend(self, entry: Any, exit: Any) -> "EntryExit":
        """Append aligned boolean event histories and return this adapter."""
        entry_array = as_bool_series(entry)
        exit_array = as_bool_series(exit)
        if entry_array.shape != exit_array.shape:
            raise ValueError("entry and exit must have equal lengths")
        self._state.extend(entry_array, exit_array)
        return self

    def compute(self) -> np.ndarray:
        """Return the complete causal position history."""
        return self._state.compute()

    @property
    def value(self) -> float | None:
        """Return the latest position or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "EntryExit":
        """Reset native state and output history, returning this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed event pairs."""
        return len(self._state.compute())
