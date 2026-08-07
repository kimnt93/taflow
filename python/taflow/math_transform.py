"""Continuous TA-Lib math transforms with descriptive ``Math`` names."""

from typing import Any

import numpy as np

from ._native import (
    StatefulAcos, StatefulAsin, StatefulAtan, StatefulCeil, StatefulCos,
    StatefulCosh, StatefulExp, StatefulFloor, StatefulLn, StatefulLog10,
    StatefulSin, StatefulSinh, StatefulSqrt, StatefulTan, StatefulTanh,
    StatefulAdd, StatefulSub, StatefulMult, StatefulDiv,
)
from ._series import as_float64_series


class _MathUnary:
    """Native-backed pointwise state with aligned history."""

    _native_cls = None

    def __init__(self, _input: Any | None = None) -> None:
        self._state = self._native_cls()
        self._values: list[float] = []
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        value = self._state.append(float(_input))
        self._values.append(np.nan if value is None else float(value))
        return self

    def extend(self, _input: Any):
        values = self._state.extend(as_float64_series(_input))
        self._values.extend(np.asarray(values, dtype=np.float64).tolist())
        return self

    def compute(self) -> np.ndarray:
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        self._values.clear()
        return self


class _MathBinary:
    _native_cls = None

    def __init__(self, left: Any | None = None, right: Any | None = None) -> None:
        self._state = self._native_cls()
        self._values: list[float] = []
        if left is not None and right is not None:
            self.extend(left, right)

    def append(self, left: float, right: float):
        value = self._state.append(float(left), float(right))
        self._values.append(float(value))
        return self

    def extend(self, left: Any, right: Any):
        a, b = as_float64_series(left), as_float64_series(right)
        if len(a) != len(b):
            raise ValueError("inputs must have equal lengths")
        for x, y in zip(a, b):
            self.append(x, y)
        return self

    def compute(self) -> np.ndarray:
        return np.asarray(self._values, dtype=np.float64)

    @property
    def value(self): return self._state.value

    def reset(self):
        self._state.reset(); self._values.clear(); return self


class MathAcos(_MathUnary): _native_cls = StatefulAcos
class MathAsin(_MathUnary): _native_cls = StatefulAsin
class MathAtan(_MathUnary): _native_cls = StatefulAtan
class MathCeil(_MathUnary): _native_cls = StatefulCeil
class MathCos(_MathUnary): _native_cls = StatefulCos
class MathCosh(_MathUnary): _native_cls = StatefulCosh
class MathExp(_MathUnary): _native_cls = StatefulExp
class MathFloor(_MathUnary): _native_cls = StatefulFloor
class MathLn(_MathUnary): _native_cls = StatefulLn
class MathLog10(_MathUnary): _native_cls = StatefulLog10
class MathSin(_MathUnary): _native_cls = StatefulSin
class MathSinh(_MathUnary): _native_cls = StatefulSinh
class MathSqrt(_MathUnary): _native_cls = StatefulSqrt
class MathTan(_MathUnary): _native_cls = StatefulTan
class MathTanh(_MathUnary): _native_cls = StatefulTanh
class MathAdd(_MathBinary): _native_cls = StatefulAdd
class MathSubtract(_MathBinary): _native_cls = StatefulSub
class MathMultiply(_MathBinary): _native_cls = StatefulMult
class MathDivide(_MathBinary): _native_cls = StatefulDiv


__all__ = [name for name in globals() if name.startswith("Math")]
