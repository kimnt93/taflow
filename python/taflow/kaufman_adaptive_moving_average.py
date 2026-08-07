"""Canonical Kaufman Adaptive Moving Average adapter."""

from ._native import StatefulKama
from ._unary_state import UnaryStateAdapter


class KaufmanAdaptiveMovingAverage(UnaryStateAdapter):
    """Compute KAMA through the native Rust state."""

    _native_cls = StatefulKama
