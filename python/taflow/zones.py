"""Bounded active-zone storage used by causal zone indicators."""
from ._native import ActiveZoneListOperator as _Native


class ActiveZoneList:
    """Stateful ActiveZoneList indicator.
    Parameters are documented by the constructor signature; scalar
    ``append`` returns the current value and ``compute`` returns
    the aligned history with NaN warm-up where applicable.
    """
    def __init__(self, capacity: int = 64):
        self._state = _Native(capacity)

    def add(self, top: float, bottom: float, flags: int = 0):
        return self._state.add(top, bottom, flags)

    def advance(self, price: float, max_age: int | None = None):
        return self._state.advance(price, max_age)

    @property
    def size(self):
        return self._state.size

    def reset(self):
        self._state.reset()
        return self
