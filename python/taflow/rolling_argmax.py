"""Canonical rolling argmax adapter."""

from ._native import StatefulMaxindex
from ._unary_state import UnaryStateAdapter


class RollingArgmax(UnaryStateAdapter):
    """Compute trailing maximum indices through the native Rust state.

    Parameters
    ----------
    timeperiod : int
        Number of observations in each trailing window.
    input_values : array-like, optional
        Initial chronological values processed by the native state.

    Returns
    -------
    RollingArgmax
        A persistent native-backed rolling operator.
    """

    _native_cls = StatefulMaxindex
