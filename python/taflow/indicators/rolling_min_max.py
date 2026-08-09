"""Persistent RollingMinMax interface."""

from typing import Any

from .._rolling_pair_state import RollingPairState
from .._native import RollingMinMax as _NativeRollingMinMax


class RollingMinMax(RollingPairState):
    """Compute RollingMinMax over a required series in native Rust state.

    ``timeperiod`` defaults to 30. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = _NativeRollingMinMax

    def append(self, _input: float) -> "RollingMinMax":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMinMax":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMinMax":
        """Reset native state and return this indicator."""
        super().reset()
        return self
