"""Canonical normalized Average True Range adapter."""

from ._native import StatefulNatr
from ._ohlc_state import OhlcStateAdapter


class NormalizedAverageTrueRange(OhlcStateAdapter):
    """Compute normalized ATR through the native Rust kernel."""

    _native_cls = StatefulNatr
