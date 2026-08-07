"""Canonical Average True Range adapter."""

from ._native import StatefulAtr
from ._ohlc_state import OhlcStateAdapter


class AverageTrueRange(OhlcStateAdapter):
    """Compute Wilder's average true range through Rust."""

    _native_cls = StatefulAtr
