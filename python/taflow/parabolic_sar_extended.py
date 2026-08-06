"""Descriptive stateful interface for extended Parabolic SAR."""

from taflow._native import StatefulSarext


class ParabolicSarExtended:
    """Incrementally compute signed SAREXT with independent trend settings."""

    def __init__(
        self,
        start_value=0.0,
        offset_on_reverse=0.0,
        acceleration_init_long=0.02,
        acceleration_long=0.02,
        acceleration_max_long=0.2,
        acceleration_init_short=0.02,
        acceleration_short=0.02,
        acceleration_max_short=0.2,
    ):
        self._state = StatefulSarext(
            start_value,
            offset_on_reverse,
            acceleration_init_long,
            acceleration_long,
            acceleration_max_long,
            acceleration_init_short,
            acceleration_short,
            acceleration_max_short,
        )

    def append(self, high, low):
        return self._state.append(high, low)

    def extend(self, high, low):
        return self._state.extend(high, low)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
