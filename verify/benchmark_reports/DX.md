# DirectionalMovementIndex benchmark (`DX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | TA-Lib ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.056 | 18.00M | 0.012 | 85.56M | 0.040 | 0.73× | 3.46× |
| 10,000 | 0.506 | 19.75M | 0.107 | 93.82M | 0.120 | 0.24× | 1.12× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
