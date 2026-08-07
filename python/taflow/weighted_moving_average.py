"""Canonical Weighted Moving Average adapter."""

from ._native import StatefulWma
from ._unary_state import UnaryStateAdapter


class WeightedMovingAverage(UnaryStateAdapter):
    """Compute the linearly weighted moving average using Rust."""

    _native_cls = StatefulWma
