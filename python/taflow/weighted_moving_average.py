"""Canonical Weighted Moving Average adapter."""

from ._native import StatefulWma
from ._unary_state import UnaryStateAdapter


class WeightedMovingAverage(UnaryStateAdapter):
    """Compute the linearly weighted moving average using Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    WeightedMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulWma
