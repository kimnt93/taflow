"""Canonical fractional Rate of Change adapter."""

from ._native import StatefulRocp
from ._unary_state import UnaryStateAdapter


class RateOfChangePercent(UnaryStateAdapter):
    """Compute fractional rate of change through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    RateOfChangePercent
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulRocp
