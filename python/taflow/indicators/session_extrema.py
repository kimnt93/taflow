"""Native-backed causal session-extrema adapter."""

from typing import Any

import numpy as np

from .._adapter_protocol import adapter_length
from .._native import SessionExtremaOperator as _Native
from .._series import as_bool_series, as_float64_series


class SessionExtrema:
    """Track high and low values within explicit causal sessions.

    ``new_session`` is a required boolean series followed by required
    ``high`` and ``low`` histories; all three may be empty for a fresh stream.
    A true flag resets the extrema at that bar. ``compute`` returns
    ``(session_high, session_low)`` arrays, ``value`` exposes the latest tuple
    or ``None`` for an empty stream, and lifecycle mutators return ``self``.
    """

    def __init__(self, new_session: Any, high: Any, low: Any) -> None:
        self._state = _Native()
        self.extend(new_session, high, low)

    def append(self, new_session: bool, high: float, low: float) -> "SessionExtrema":
        """Append one session/high/low bar and return this adapter."""
        self._state.append(bool(new_session), float(high), float(low))
        return self

    def extend(self, new_session: Any, high: Any, low: Any) -> "SessionExtrema":
        """Append equal-length session/high/low histories."""
        arrays = (
            as_bool_series(new_session),
            as_float64_series(high),
            as_float64_series(low),
        )
        if len({len(array) for array in arrays}) != 1:
            raise ValueError("new_session, high, and low must have equal lengths")
        self._state.extend(*arrays)
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned session-high and session-low arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[float, float] | None:
        """Return the latest extrema tuple, or ``None`` for an empty stream."""
        return self._state.value

    def reset(self) -> "SessionExtrema":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed bars."""
        return adapter_length(self)


__all__ = ["SessionExtrema"]
