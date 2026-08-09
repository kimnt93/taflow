"""Bounded active-zone storage used by causal zone indicators."""

from numpy.typing import NDArray

from ._native import ActiveZoneListOperator as _Native


class ActiveZoneList:
    """Bounded zone storage with causal invalidation and fluent reset."""

    def __init__(self, capacity: int = 64) -> None:
        """Create a list retaining at most ``capacity`` zones."""
        self._state = _Native(capacity)

    def add(self, top: float, bottom: float, flags: int = 0) -> int:
        """Store a normalized zone and return its native index."""
        return self._state.add(float(top), float(bottom), int(flags))

    def advance(self, price: float, max_age: int | None = None) -> NDArray:
        """Advance one price observation and return invalidation flags."""
        return self._state.advance(float(price), max_age)

    @property
    def size(self) -> int:
        """Return the number of active zones."""
        return self._state.size

    def reset(self) -> "ActiveZoneList":
        """Clear all zones and return this adapter."""
        self._state.reset()
        return self

    def __len__(self) -> int:
        """Return the number of active zones."""
        return self.size
