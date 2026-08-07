"""Canonical Double Exponential Moving Average adapter."""

from ._native import StatefulDema
from ._unary_state import UnaryStateAdapter


class DoubleExponentialMovingAverage(UnaryStateAdapter):
    """Compute DEMA through the native Rust state

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    DoubleExponentialMovingAverage
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulDema
