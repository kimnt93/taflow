"""Native-backed Premium/Discount adapter."""

from typing import Any

import numpy as np

from .._native import PremiumDiscount as _NativePremiumDiscount
from .._series import as_float64_series


class PremiumDiscount:
    """Compute rolling equilibrium and signed premium/discount levels.

    Parameters
    ----------
    close : array-like
        Chronological close series supplied through ``extend``.
    window : int, default 20
        Positive rolling window used by the native Rust state.

    ``compute`` returns ``(zone, equilibrium)`` arrays in that order, with
    Rust-owned causal values. ``value`` exposes the latest pair or ``None``
    before the first bar. ``append``, ``extend``, and ``reset`` mutate and
    return this adapter. No independent external oracle is available; this is
    a TAFlow session-zone definition.
    """

    def __init__(self, window: int = 20) -> None:
        self._state = _NativePremiumDiscount(int(window))

    def append(self, close: float) -> "PremiumDiscount":
        """Append one chronological close and return this adapter."""
        self._state.append(float(close))
        return self

    def extend(self, close: Any) -> "PremiumDiscount":
        """Append a converted chronological close history."""
        self._state.extend(as_float64_series(close))
        return self

    def compute(self) -> tuple[np.ndarray, np.ndarray]:
        """Return aligned level and premium/discount arrays."""
        return self._state.compute()

    @property
    def value(self) -> tuple[int, float] | None:
        """Return the latest pair, or ``None`` during warm-up."""
        return self._state.value

    def reset(self) -> "PremiumDiscount":
        """Restore fresh native state and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of processed close values."""
        return len(self._state)


__all__ = ["PremiumDiscount"]
