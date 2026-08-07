"""Canonical Triangular Moving Average adapter."""

from ._native import StatefulTrima
from ._unary_state import UnaryStateAdapter


class TriangularMovingAverage(UnaryStateAdapter):
    """Compute the triangular moving average through Rust."""

    _native_cls = StatefulTrima
