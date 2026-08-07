"""Canonical Simple Moving Average adapter."""

from ._native import StatefulSma
from ._unary_state import UnaryStateAdapter


class SimpleMovingAverage(UnaryStateAdapter):
    """Compute the simple moving average using the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    SimpleMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulSma
