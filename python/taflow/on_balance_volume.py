"""Canonical On-Balance Volume adapter."""

from ._native import StatefulObv
from ._volume_state import CloseVolumeStateAdapter


class OnBalanceVolume(CloseVolumeStateAdapter):
    """Compute On-Balance Volume through the native Rust state."""

    _native_cls = StatefulObv
