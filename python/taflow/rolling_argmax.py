"""Canonical rolling argmax adapter."""

from ._native import StatefulMaxindex
from ._unary_state import UnaryStateAdapter


class RollingArgmax(UnaryStateAdapter):
    """Compute trailing maximum indices through the native Rust state."""

    _native_cls = StatefulMaxindex
