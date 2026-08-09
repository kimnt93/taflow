"""Persistent RollingMinMaxIndex interface."""

from typing import Any

from .._rolling_pair_state import RollingPairState
from .._native import RollingMinMaxIndex as _NativeRollingMinMaxIndex


class RollingMinMaxIndex(RollingPairState):
    """Compute RollingMinMaxIndex over a required series in native Rust state.

    ``timeperiod`` defaults to 30. History is aligned and contains NaN
    until the trailing window is complete.
    """

    _native_cls = _NativeRollingMinMaxIndex

    def append(self, _input: float) -> "RollingMinMaxIndex":
        """Append one value and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "RollingMinMaxIndex":
        """Append a chronological series and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "RollingMinMaxIndex":
        """Reset native state and return this indicator."""
        super().reset()
        return self
