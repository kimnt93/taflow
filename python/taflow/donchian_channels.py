"""Canonical descriptive alias for rolling Donchian channels."""

from typing import Any

from .donchian import Donchian


class DonchianChannels(Donchian):
    """Canonical descriptive name for the persistent Donchian channel state."""

    def append(self, high: float, low: float) -> "DonchianChannels":
        super().append(high, low)
        return self

    def extend(self, high: Any, low: Any) -> "DonchianChannels":
        super().extend(high, low)
        return self

    def reset(self) -> "DonchianChannels":
        super().reset()
        return self

__all__ = ["DonchianChannels"]
