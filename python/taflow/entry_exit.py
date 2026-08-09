"""Native-backed entry/exit signal state."""

from typing import Any

import numpy as np

from ._native import EntryExitOperator


class EntryExit:
    """Track entry and exit events as a stateful position signal.

    Parameters
    ----------
    entry : array-like
        Initial entry-event history.
    _exit : array-like
        Initial exit-event history.
    """

    def __init__(
        self,
        entry: Any,
        _exit: Any,
    ) -> None:
        """Create the state and process both signal histories."""
        self._state = EntryExitOperator()
        if entry is not None or _exit is not None:
            self.extend(entry, _exit)

    def append(self, entry: bool, _exit: bool) -> "EntryExit":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        entry : bool
            Current entry condition.
        _exit : bool
            Current exit condition.

        Returns
        -------
        EntryExit
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(entry, _exit)
        return self

    def extend(self, entry: Any, _exit: Any) -> "EntryExit":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        entry : Any
            Chronological entry condition series.
        _exit : Any
            Chronological exit condition series.

        Returns
        -------
        EntryExit
            This indicator, for fluent chaining."""
        self._state.extend(np.asarray(entry, dtype=bool), np.asarray(_exit, dtype=bool))
        return self

    def compute(self) -> np.ndarray:
        """Return the complete aligned history produced by Rust.

        Returns
        -------
        numpy.ndarray or tuple of numpy.ndarray
            One output per processed bar, including NaN warm-up positions."""
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest Rust result.

        Returns
        -------
        float, tuple, or None
            Latest output, or None while scalar warm-up is incomplete."""
        return self._state.value

    def reset(self) -> "EntryExit":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        EntryExit
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
