"""Descriptive stateful interface for the Stochastic Oscillator."""

from taflow._native import StatefulStoch


class StochasticOscillator:
    """Incrementally compute aligned slow %K and slow %D."""

    def __init__(
        self,
        fast_k_period=5,
        slow_k_period=3,
        slow_k_average_type=0,
        slow_d_period=3,
        slow_d_average_type=0,
    ):
        self._state = StatefulStoch(
            fast_k_period,
            slow_k_period,
            slow_k_average_type,
            slow_d_period,
            slow_d_average_type,
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
