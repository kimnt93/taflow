"""Session-boundary helper compatibility surface."""

from typing import Any

import numpy as np

from ._native import session_flags_array as _native_session_flags
from .session_extrema import SessionExtrema


def session_flags(session_id: Any) -> np.ndarray:
    """Return true at the first bar and whenever the session identifier changes."""
    return _native_session_flags(np.asarray(session_id, dtype=np.float64))


__all__ = ["SessionExtrema", "session_flags"]
