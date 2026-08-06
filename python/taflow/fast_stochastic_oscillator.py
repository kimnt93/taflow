"""Descriptive stateful interface for the Fast Stochastic Oscillator."""

from taflow._native import StatefulStochf


class FastStochasticOscillator:
    """Incrementally compute aligned fast %K and fast %D."""

    def __init__(self, fast_k_period=5, fast_d_period=3, fast_d_average_type=0):
        self._state = StatefulStochf(
            fast_k_period,
            fast_d_period,
            fast_d_average_type,
        )

    def append(self, high, low, close):
        return self._state.append(high, low, close)

    def extend(self, high, low, close):
        return self._state.extend(high, low, close)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
