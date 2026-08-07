"""Canonical rolling sum adapter."""

from ._native import StatefulSum
from ._unary_state import UnaryStateAdapter


class RollingSum(UnaryStateAdapter):
    """Compute the trailing sum through the native Rust state."""

    _native_cls = StatefulSum
