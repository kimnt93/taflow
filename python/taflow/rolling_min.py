"""Canonical rolling minimum adapter."""

from ._native import StatefulMin
from ._unary_state import UnaryStateAdapter


class RollingMin(UnaryStateAdapter):
    """Compute the trailing minimum through the native Rust state."""

    _native_cls = StatefulMin
