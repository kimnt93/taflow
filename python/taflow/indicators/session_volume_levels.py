"""Native-backed session volume-profile adapter."""

from typing import Any

import numpy as np

from .._native import SessionVolumeLevels as _NativeSessionVolumeLevels
from .._series import as_bool_series, as_float64_series


class SessionVolumeLevels:
    """Compute session point-of-control and value-area levels.

    ``high``, ``low``, ``close``, ``volume``, and boolean ``anchor`` are
    supplied to ``extend`` as aligned histories in that order. ``bins`` defaults to 24 and ``value_area`` to 0.7. ``compute``
    returns ``(point_of_control, value_area_high, value_area_low)`` arrays.
    Rust owns binning, session resets, warm-up, and output alignment; lifecycle
    mutators return ``self`` and reject unequal lengths before mutation.
    """

    def __init__(self, bins: int = 24, value_area: float = 0.7) -> None:
        self._state = _NativeSessionVolumeLevels(int(bins), float(value_area))

    def append(
        self,
        high: float,
        low: float,
        close: float,
        volume: float,
        anchor: bool,
    ) -> "SessionVolumeLevels":
        """Append one high/low/close/volume/anchor bar in that order."""
        self._state.append(
            float(high), float(low), float(close), float(volume), bool(anchor)
        )
        return self

    def extend(
        self, high: Any, low: Any, close: Any, volume: Any, anchor: Any
    ) -> "SessionVolumeLevels":
        """Append aligned high/low/close/volume/anchor histories in that order."""
        arrays = (
            as_float64_series(high),
            as_float64_series(low),
            as_float64_series(close),
            as_float64_series(volume),
            as_bool_series(anchor),
        )
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("high, low, close, volume, and anchor must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray, np.ndarray]:
        """Return aligned point-of-control and value-area arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float, float] | None:
        """Return the latest three-level tuple, or ``None`` before warm-up."""
        return self._state.value

    def reset(self) -> "SessionVolumeLevels":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return len(self._state)


__all__ = ["SessionVolumeLevels"]
