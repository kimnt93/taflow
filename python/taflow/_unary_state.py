"""Shared native-backed lifecycle adapter for unary indicators."""

from typing import Any

import numpy as np

from ._series import as_float64_series


class UnaryStateAdapter:
    """Adapt a native unary state without performing numerical work in Python."""

    _native_cls = None

    def __init__(self, _input: Any | None = None, timeperiod: int = 14) -> None:
        """Create the native state and optionally process an input history."""
        if self._native_cls is None:
            raise TypeError("a native state class must be configured")
        self._state = self._native_cls(timeperiod)
        self._values: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        """Append one value and update the native state."""
        value = self._state.append(float(_input))
        self._values.append(np.nan if value is None else value)
        return self

    def extend(self, _input: Any):
        """Append an aligned input history to the native state."""
        values = self._state.extend(as_float64_series(_input))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        """Return the aligned native output history."""
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self):
        """Return the latest native output, or ``None`` during warm-up."""
        return self._state.value

    def reset(self):
        """Reset native state and accumulated output history."""
        self._state.reset()
        self._values.clear()
        return self
