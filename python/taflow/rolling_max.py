"""Canonical rolling maximum adapter."""

from ._native import StatefulMax
from ._unary_state import UnaryStateAdapter


class RollingMax(UnaryStateAdapter):
    """Compute the trailing maximum through the native Rust state."""

    _native_cls = StatefulMax
