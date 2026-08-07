"""Canonical normalized Average True Range adapter."""

from ._native import StatefulNatr
from ._ohlc_state import OhlcStateAdapter


class NormalizedAverageTrueRange(OhlcStateAdapter):
    """Compute normalized ATR through the native Rust kernel

    Parameters
    ----------
    Input series and configuration values are accepted by the constructor.

    Returns
    -------
    NormalizedAverageTrueRange
        A persistent native-backed indicator adapter.
    """

    _native_cls = StatefulNatr
