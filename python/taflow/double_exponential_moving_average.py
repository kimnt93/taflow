"""Canonical Double Exponential Moving Average adapter."""

from ._native import StatefulDema
from ._unary_state import UnaryStateAdapter


class DoubleExponentialMovingAverage(UnaryStateAdapter):
    """Compute DEMA through the native Rust state."""

    _native_cls = StatefulDema
