"""Canonical Accumulation/Distribution adapter."""

from ._native import StatefulAd
from ._volume_state import OhlcvStateAdapter


class AccumulationDistribution(OhlcvStateAdapter):
    """Compute cumulative Accumulation/Distribution through Rust."""

    _native_cls = StatefulAd
