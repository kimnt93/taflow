"""Persistent RollingBeta interface."""

from typing import Any

from ._bivariate_state import BivariateState
from ._native import StatefulBeta


class RollingBeta(BivariateState):
    """Compute RollingBeta from two required aligned series in native Rust state.

    ``timeperiod`` defaults to 5; history is NaN until warm-up completes.
    """

    _native_cls = StatefulBeta

    def append(self, input0: float, input1: float) -> "RollingBeta":
        """Append one aligned pair and return this indicator."""
        super().append(input0, input1)
        return self

    def extend(self, input0: Any, input1: Any) -> "RollingBeta":
        """Append aligned histories and return this indicator."""
        super().extend(input0, input1)
        return self

    def reset(self) -> "RollingBeta":
        """Reset native state and return this indicator."""
        super().reset()
        return self
