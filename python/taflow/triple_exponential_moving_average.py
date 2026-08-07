"""Canonical Triple Exponential Moving Average adapter."""

from ._native import StatefulTema
from ._unary_state import UnaryStateAdapter


class TripleExponentialMovingAverage(UnaryStateAdapter):
    """Compute TEMA through the native Rust state."""

    _native_cls = StatefulTema
