"""Canonical Simple Moving Average adapter."""

from ._native import StatefulSma
from ._unary_state import UnaryStateAdapter


class SimpleMovingAverage(UnaryStateAdapter):
    """Compute the simple moving average using the native Rust kernel."""

    _native_cls = StatefulSma
