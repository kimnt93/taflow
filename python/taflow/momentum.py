"""Canonical Momentum adapter."""

from ._native import StatefulMom
from ._unary_state import UnaryStateAdapter


class Momentum(UnaryStateAdapter):
    """Compute price momentum through the native Rust state."""

    _native_cls = StatefulMom
