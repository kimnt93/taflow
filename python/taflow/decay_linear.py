"""Linearly decayed moving average."""
from typing import Any

from .weighted_moving_average import WeightedMovingAverage


class DecayLinear(WeightedMovingAverage):
    """WorldQuant linear decay, exactly equivalent to weighted moving average.

    The class reuses the native ``WeightedMovingAverage`` state and preserves
    its constructor, append, extend, compute, value, and reset lifecycle.
    """

    def append(self, _input: float) -> "DecayLinear":
        """Append one observation and return this indicator."""
        super().append(_input)
        return self

    def extend(self, _input: Any) -> "DecayLinear":
        """Append aligned histories and return this indicator."""
        super().extend(_input)
        return self

    def reset(self) -> "DecayLinear":
        """Reset native state and return this indicator."""
        super().reset()
        return self
