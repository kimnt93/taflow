"""Canonical rolling argmin adapter."""

from ._native import StatefulMinindex
from ._unary_state import UnaryStateAdapter


class RollingArgmin(UnaryStateAdapter):
    """Compute trailing minimum indices through the native Rust state."""

    _native_cls = StatefulMinindex
