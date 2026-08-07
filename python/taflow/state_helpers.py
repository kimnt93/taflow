"""Compatibility exports for stateful signal helpers.

The concrete lifecycle adapters live in one module per public indicator;
the three condition/value helpers retain a shared native-backed factory.
"""

from typing import Any

import numpy as np

from ._native import HighestSinceOperator, LowestSinceOperator, ValueWhenOperator
from ._series import as_float64_series
from .bars_since import BarsSince
from .entry_exit import EntryExit
from .position_hold import PositionHold
from .signal_delay import SignalDelay


def _make(native: object, name: str) -> type:
    """Create a native-backed two-input state adapter class."""

    class State:
        """Track a condition and associated value series.

        Parameters
        ----------
        condition, input_values : array-like, optional
            Initial aligned condition and value histories.

        Returns
        -------
        State
            A persistent native-backed signal helper.
        """

        def __init__(
            self, condition: Any | None = None, _input: Any | None = None
        ) -> None:
            """Create the state and optionally process aligned histories."""
            self._state = native()
            if condition is not None or _input is not None:
                self.extend(condition, _input)

        def append(self, condition: bool, _input: float) -> "State":
            """Append one condition/value pair to the native state

            Parameters
            ----------
            values : object
                Input values or the aligned result container.

            Returns
            -------
            object
                Updated state, converted values, or aligned output.
            """
            self._state.append(condition, _input)
            return self

        def extend(self, condition: Any, _input: Any) -> "State":
            """Process aligned condition and value histories in native Rust

            Parameters
            ----------
            values : object
                Input values or the aligned result container.

            Returns
            -------
            object
                Updated state, converted values, or aligned output.
            """
            self._state.extend(
                np.asarray(condition, dtype=bool), as_float64_series(_input)
            )
            return self

        def compute(self) -> np.ndarray:
            """Return the aligned native output history

            Returns
            -------
            object
                Updated state, converted values, or aligned output.
            """
            return self._state.compute()

        @property
        def value(self) -> object:
            """Return the latest native output

            Returns
            -------
            object
                Updated state, converted values, or aligned output.
            """
            return self._state.value

        def reset(self) -> "State":
            """Reset the native state and accumulated history

            Returns
            -------
            object
                Updated state, converted values, or aligned output.
            """
            self._state.reset()
            return self

    State.__name__ = name
    State.__qualname__ = name
    State.__module__ = __name__
    return State


ValueWhen = _make(ValueWhenOperator, "ValueWhen")
HighestSince = _make(HighestSinceOperator, "HighestSince")
LowestSince = _make(LowestSinceOperator, "LowestSince")

__all__ = [
    "BarsSince",
    "ValueWhen",
    "HighestSince",
    "LowestSince",
    "SignalDelay",
    "PositionHold",
    "EntryExit",
]
