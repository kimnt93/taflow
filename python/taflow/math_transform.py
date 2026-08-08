"""Continuous TA-Lib math transforms with descriptive ``Math`` names."""

from typing import Any

import numpy as np

from ._native import (
    StatefulMathAbs, StatefulMathAcos, StatefulMathAcosh, StatefulMathAdd,
    StatefulMathAsin, StatefulMathAsinh, StatefulMathAtan, StatefulMathAtanh,
    StatefulMathCbrt, StatefulMathCeil, StatefulMathCos, StatefulMathCosh,
    StatefulMathCot, StatefulMathDegrees, StatefulMathDivide, StatefulMathExp,
    StatefulMathFloor, StatefulMathLn, StatefulMathLog10, StatefulMathLog1p,
    StatefulMathMultiply, StatefulMathRadians, StatefulMathSin,
    StatefulMathSinh, StatefulMathSqrt, StatefulMathSubtract, StatefulMathTan,
    StatefulMathTanh,
)
from ._series import as_float64_series


class _MathUnary:
    """Native-backed pointwise state with aligned history."""

    _native_cls = None

    def __init__(self, _input: Any | None = None) -> None:
        self._state = self._native_cls()
        if _input is not None:
            self.extend(_input)

    def append(self, _input: float):
        self._state.append(float(_input))
        return self

    def extend(self, _input: Any):
        self._state.extend(as_float64_series(_input))
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self):
        return self._state.value

    def reset(self):
        self._state.reset()
        return self

    def __len__(self) -> int:
        return len(self._state)


class _MathBinary:
    _native_cls = None

    def __init__(self, left: Any | None = None, right: Any | None = None) -> None:
        self._state = self._native_cls()
        if left is not None and right is not None:
            self.extend(left, right)

    def append(self, left: float, right: float):
        self._state.append(float(left), float(right))
        return self

    def extend(self, left: Any, right: Any):
        a, b = as_float64_series(left), as_float64_series(right)
        if len(a) != len(b):
            raise ValueError("inputs must have equal lengths")
        self._state.extend(a, b)
        return self

    def compute(self) -> np.ndarray:
        return self._state.compute()

    @property
    def value(self): return self._state.value

    def __len__(self) -> int:
        return len(self._state)

    def reset(self):
        self._state.reset(); return self


class MathAbs(_MathUnary): _native_cls = StatefulMathAbs
class MathAcos(_MathUnary): _native_cls = StatefulMathAcos
class MathAcosh(_MathUnary): _native_cls = StatefulMathAcosh
class MathAsin(_MathUnary): _native_cls = StatefulMathAsin
class MathAsinh(_MathUnary): _native_cls = StatefulMathAsinh
class MathAtan(_MathUnary): _native_cls = StatefulMathAtan
class MathAtanh(_MathUnary): _native_cls = StatefulMathAtanh
class MathCbrt(_MathUnary): _native_cls = StatefulMathCbrt
class MathCeil(_MathUnary): _native_cls = StatefulMathCeil
class MathCos(_MathUnary): _native_cls = StatefulMathCos
class MathCosh(_MathUnary): _native_cls = StatefulMathCosh
class MathCot(_MathUnary): _native_cls = StatefulMathCot
class MathDegrees(_MathUnary): _native_cls = StatefulMathDegrees
class MathExp(_MathUnary): _native_cls = StatefulMathExp
class MathFloor(_MathUnary): _native_cls = StatefulMathFloor
class MathLn(_MathUnary): _native_cls = StatefulMathLn
class MathLog10(_MathUnary): _native_cls = StatefulMathLog10
class MathLog1p(_MathUnary): _native_cls = StatefulMathLog1p
class MathRadians(_MathUnary): _native_cls = StatefulMathRadians
class MathSin(_MathUnary): _native_cls = StatefulMathSin
class MathSinh(_MathUnary): _native_cls = StatefulMathSinh
class MathSqrt(_MathUnary): _native_cls = StatefulMathSqrt
class MathTan(_MathUnary): _native_cls = StatefulMathTan
class MathTanh(_MathUnary): _native_cls = StatefulMathTanh
class MathAdd(_MathBinary): _native_cls = StatefulMathAdd
class MathSubtract(_MathBinary): _native_cls = StatefulMathSubtract
class MathMultiply(_MathBinary): _native_cls = StatefulMathMultiply
class MathDivide(_MathBinary): _native_cls = StatefulMathDivide


__all__ = [name for name in globals() if name.startswith("Math")]
