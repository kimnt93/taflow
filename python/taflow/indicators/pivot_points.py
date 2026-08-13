"""Native-backed anchored pivot-point adapter."""

from typing import Any

import numpy as np

from .._native import PivotPoints as _NativePivotPoints
from .._series import as_bool_series, as_float64_series


class PivotPoints:
    """Compute anchored classic pivot, resistance, and support levels.

    ``high``, ``low``, ``close``, and boolean ``anchor`` are required aligned
    histories in that order; supply the aligned series through ``extend`` after construction. ``compute``
    returns ``(pivot, resistance_one, support_one, support_two,
    resistance_two)`` arrays. Rust owns causal anchor transitions and warm-up;
    ``value`` exposes the latest five-value tuple or ``None``. Lifecycle
    mutators are fluent and reject unequal input lengths before mutation.
    """

    def __init__(self) -> None:
        self._state = _NativePivotPoints()

    def append(
        self, high: float, low: float, close: float, anchor: bool
    ) -> "PivotPoints":
        """Append one high/low/close/anchor bar in that order."""
        self._state.append(float(high), float(low), float(close), bool(anchor))
        return self

    def extend(
        self, high: Any, low: Any, close: Any, anchor: Any
    ) -> "PivotPoints":
        """Append aligned high/low/close/anchor histories in that order."""
        arrays = (
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_bool_series(anchor),
        )
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, close, and anchor must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(
        self,
    ) -> tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned pivot and four support/resistance arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float, float, float] | None:
        """Return the latest five-level tuple, or ``None`` before warm-up."""
        return self._state.value

    def reset(self) -> "PivotPoints":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["PivotPoints"]
