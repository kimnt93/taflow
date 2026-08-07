"""Native-backed bars-since signal state."""

from typing import Any

import numpy as np

from ._native import BarsSinceOperator


class BarsSince:
    """Count bars since the most recent true condition.

    Parameters
    ----------
    condition : array-like, optional
        Initial boolean condition history.
    """

    def __init__(self, condition: Any | None = None) -> None:
        """Create the state and optionally process condition history."""
        self._state = BarsSinceOperator()
        if condition is not None:
            self.extend(condition)

    def append(self, condition: bool) -> "BarsSince":
        """Append one condition and update the native result..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.append(condition)
        return self

    def extend(self, condition: Any) -> "BarsSince":
        """Process an aligned boolean condition series in native Rust..

        Parameters
        ----------
        values : object
            Input values or the aligned result container.

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.extend(np.asarray(condition, dtype=bool))
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned bars-since history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.compute()

    @property
    def value(self) -> object:
        """Return the latest bars-since value..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        return self._state.value

    def reset(self) -> "BarsSince":
        """Reset the native state and accumulated history..

        Returns
        -------
        object
            Updated state, converted values, or aligned output.
        """
        self._state.reset()
        return self
