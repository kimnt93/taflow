"""Linearly decayed moving average."""

from .weighted_moving_average import WeightedMovingAverage


class DecayLinear(WeightedMovingAverage):
    """WorldQuant linear decay, exactly equivalent to weighted moving average.

    The class reuses the native ``WeightedMovingAverage`` state and preserves
    its constructor, append, extend, compute, value, and reset lifecycle.
    """
