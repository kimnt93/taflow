"""Native-backed entry/exit signal state."""

from typing import Any

import numpy as np

from ._native import EntryExitOperator


class EntryExit:
    """Track entry and exit events as a stateful position signal.

    Parameters
    ----------
    entry : array-like, optional
        Initial entry-event history.
    _exit : array-like, optional
        Initial exit-event history.
    """

    def __init__(self, entry: Any | None = None, _exit: Any | None = None) -> None:
        """Create the state and optionally process both signal histories."""
        self._state = EntryExitOperator()
        if entry is not None or _exit is not None:
            self.extend(entry, _exit)

    def append(self, entry: bool, _exit: bool) -> "EntryExit":
        """Append one entry/exit pair and update the native result

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(entry, _exit)
        return self

    def extend(self, entry: Any, _exit: Any) -> "EntryExit":
        """Process aligned entry and exit series in native Rust

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(np.asarray(entry, dtype=bool), np.asarray(_exit, dtype=bool))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned entry/exit state history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest entry/exit state

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> "EntryExit":
        """Reset the native state and accumulated history

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
