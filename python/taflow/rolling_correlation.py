"""Persistent RollingCorrelation interface."""

from typing import Any

from ._bivariate_state import BivariateState
from ._native import StatefulCorrel


class RollingCorrelation(BivariateState):
    """Compute RollingCorrelation from two required aligned series in native Rust state.

    ``timeperiod`` defaults to 5; history is NaN until warm-up completes.
    """

    _native_cls = StatefulCorrel

    def append(self, input0: float, input1: float) -> "RollingCorrelation":
        """Append one aligned pair and return this indicator."""
        super().append(input0, input1)
        return self

    def extend(self, input0: Any, input1: Any) -> "RollingCorrelation":
        """Append aligned histories and return this indicator."""
        super().extend(input0, input1)
        return self

    def reset(self) -> "RollingCorrelation":
        """Reset native state and return this indicator."""
        super().reset()
        return self
