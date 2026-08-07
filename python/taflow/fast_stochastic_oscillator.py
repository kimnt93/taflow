"""Descriptive stateful interface for the Fast Stochastic Oscillator."""

from taflow._native import StatefulStochf
from typing import Any


class FastStochasticOscillator:
    """Incrementally compute aligned fast %K and fast %D."""

    def __init__(self, fast_k_period: int = 5, fast_d_period: int = 3,
                 fast_d_average_type: int = 0, high: Any | None = None,
                 low: Any | None = None, close: Any | None = None):
        """Create fast stochastic with optional aligned OHLC history."""
        self._state = StatefulStochf(
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
        )
        if any(value is not None for value in (high, low, close)):
            self.extend(high, low, close)

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
