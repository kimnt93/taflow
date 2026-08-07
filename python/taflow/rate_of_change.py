"""Canonical Rate of Change adapter."""

from ._native import StatefulRoc
from ._unary_state import UnaryStateAdapter


class RateOfChange(UnaryStateAdapter):
    """Compute percentage rate of change through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRoc
