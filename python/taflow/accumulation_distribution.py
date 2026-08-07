"""Canonical Accumulation/Distribution adapter."""

from ._native import StatefulAd
from ._volume_state import OhlcvStateAdapter


class AccumulationDistribution(OhlcvStateAdapter):
    """Compute cumulative Accumulation/Distribution through Rust

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    AccumulationDistribution
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulAd
