# MathRadians benchmark (`numpy.radians` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 581.02M | 0.001 | 1.22G | 0.013 | 7.41× | 15.53× |
| 10,000 | 0.006 | 1.79G | 0.003 | 3.37G | 0.025 | 4.44× | 8.35× |
| 100,000 | 0.053 | 1.90G | 0.028 | 3.61G | 0.129 | 2.44× | 4.66× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.053 | 0.068 | 1.27× |
| 1 | 5 | 0.306 | 0.295 | 0.96× |
| 1 | 10 | 0.390 | 0.584 | 1.50× |
| 10 | 1 | 0.038 | 0.052 | 1.38× |
| 10 | 5 | 0.178 | 0.267 | 1.50× |
| 10 | 10 | 0.392 | 0.593 | 1.51× |
| 100 | 1 | 0.040 | 0.058 | 1.46× |
| 100 | 5 | 0.182 | 0.270 | 1.48× |
| 100 | 10 | 0.355 | 0.550 | 1.55× |
| 1,000 | 1 | 0.041 | 0.057 | 1.39× |
| 1,000 | 5 | 0.171 | 0.276 | 1.62× |
| 1,000 | 10 | 0.370 | 0.618 | 1.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
