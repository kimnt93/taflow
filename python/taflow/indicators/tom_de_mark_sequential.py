"""Canonical native-backed Tom DeMark Sequential adapter."""

from typing import Any

import numpy as np

from .._native import TomDeMarkSequential as _NativeTomDeMarkSequential
from .._series import as_float64_series


class TomDeMarkSequential:
    """Compute causal four-bar buy and sell setup counts.

    ``close`` is required and may be empty for a fresh stream. Rust owns the
    four-bar comparison, capped setup counters, warm-up, and aligned integer
    outputs. ``compute`` returns ``(buy, sell)`` arrays; lifecycle mutators are
    fluent. The independent oracle is ``pandas-ta-classic.td_sequential``.
    """

    def __init__(self, close: Any) -> None:
        self._state = _NativeTomDeMarkSequential()
        self.extend(close)

    def append(self, close: float) -> "TomDeMarkSequential":
        """Append one chronological close and return this adapter."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "TomDeMarkSequential":
        """Append a converted chronological close history."""
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned buy and sell setup arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[int, int] | None:
        """Return the latest setup pair, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "TomDeMarkSequential":
        """Reset the state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close values."""
        return len(self._state)


__all__ = ["TomDeMarkSequential"]
