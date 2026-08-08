# MathSqrt benchmark (`SQRT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.44M | 0.003 | 342.01M | 0.033 | 0.67× | 11.17× |
| 10,000 | 0.533 | 18.77M | 0.021 | 486.95M | 0.041 | 0.08× | 2.02× |

## Warm-up

Construct + canonical extend over 1,500 bars: **0.065 ms**; native kernel **0.004 ms**; TA-Lib 0.029 ms.

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,500 | 1 | 0.290 | 0.171 | 5.85M | 28.142 | 164.72× | 148.66× |
| 1,500 | 10 | 1.551 | 0.608 | 16.44M | 29.589 | 48.65× | 42.47× |
| 1,500 | 100 | 6.661 | 2.249 | 44.46M | 29.459 | 13.10× | 12.70× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
