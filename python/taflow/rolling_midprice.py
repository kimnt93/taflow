"""Persistent RollingMidprice interface."""

from typing import Any

from ._bivariate_state import BivariateState
from ._native import StatefulMidprice


class RollingMidprice(BivariateState):
    """Compute RollingMidprice from two required aligned series in native Rust state.

    ``timeperiod`` defaults to 14; history is NaN until warm-up completes.
    """

    _native_cls = StatefulMidprice

    def __init__(self, _input0: Any, _input1: Any, timeperiod: int = 14) -> None:
        super().__init__(_input0, _input1, timeperiod)

    def append(self, input0: float, input1: float) -> "RollingMidprice":
        """Append one aligned pair and return this indicator."""
        super().append(input0, input1)
        return self

    def extend(self, input0: Any, input1: Any) -> "RollingMidprice":
        """Append aligned histories and return this indicator."""
        super().extend(input0, input1)
        return self

    def reset(self) -> "RollingMidprice":
        """Reset native state and return this indicator."""
        super().reset()
        return self
