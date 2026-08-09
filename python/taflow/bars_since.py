"""Native-backed bars-since signal state."""

from typing import Any

import numpy as np

from ._native import BarsSinceOperator


class BarsSince:
    """Count bars since the most recent true condition.

    Parameters
    ----------
    condition : array-like
        Initial boolean condition history.
    """

    def __init__(
        self,
        condition: Any,
    ) -> None:
        """Create the state and process condition history."""
        self._state = BarsSinceOperator()
        if condition is not None:
            self.extend(condition)

    def append(self, condition: bool) -> "BarsSince":
        """Append one chronological observation to the native Rust state.

        Parameters
        ----------
        condition : bool
            Current boolean condition.

        Returns
        -------
        BarsSince
            This indicator, for fluent chaining; read `value` for the result."""
        self._state.append(condition)
        return self

    def extend(self, condition: Any) -> "BarsSince":
        """Append aligned chronological histories to the native Rust state.

        Parameters
        ----------
        condition : Any
            Chronological boolean condition series.

        Returns
        -------
        BarsSince
            This indicator, for fluent chaining."""
        self._state.extend(np.asarray(condition, dtype=bool))
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

    def reset(self) -> "BarsSince":
        """Restore fresh-state behavior and clear output history.

        Returns
        -------
        BarsSince
            This indicator, for fluent chaining."""
        self._state.reset()
        return self
