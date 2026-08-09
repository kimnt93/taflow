"""Compatibility re-exports for pointwise math classes.

Canonical classes live in one same-named module each.
"""

from .math_abs import MathAbs
from .math_acos import MathAcos
from .math_acosh import MathAcosh
from .math_add import MathAdd
from .math_asin import MathAsin
from .math_asinh import MathAsinh
from .math_atan import MathAtan
from .math_atanh import MathAtanh
from .math_cbrt import MathCbrt
from .math_ceil import MathCeil
from .math_cos import MathCos
from .math_cosh import MathCosh
from .math_cot import MathCot
from .math_degrees import MathDegrees
from .math_divide import MathDivide
from .math_exp import MathExp
from .math_floor import MathFloor
from .math_ln import MathLn
from .math_log10 import MathLog10
from .math_log1p import MathLog1p
from .math_multiply import MathMultiply
from .math_radians import MathRadians
from .math_sin import MathSin
from .math_sinh import MathSinh
from .math_sqrt import MathSqrt
from .math_subtract import MathSubtract
from .math_tan import MathTan
from .math_tanh import MathTanh

__all__ = [name for name in globals() if name.startswith("Math")]
