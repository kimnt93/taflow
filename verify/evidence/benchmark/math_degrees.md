# MathDegrees benchmark (`numpy.degrees` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 298.09M | 0.002 | 400.90M | 0.013 | 3.79× | 5.10× |
| 10,000 | 0.007 | 1.34G | 0.005 | 2.13G | 0.024 | 3.25× | 5.18× |
| 100,000 | 0.052 | 1.94G | 0.029 | 3.44G | 0.127 | 2.47× | 4.38× |
| 1,000,000 | 0.809 | 1.24G | 0.485 | 2.06G | 1.298 | 1.60× | 2.67× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.066 | 0.78× |
| 1 | 5 | 0.326 | 0.279 | 0.86× |
| 1 | 10 | 0.457 | 0.576 | 1.26× |
| 10 | 1 | 0.048 | 0.055 | 1.14× |
| 10 | 5 | 0.215 | 0.285 | 1.32× |
| 10 | 10 | 0.469 | 0.589 | 1.26× |
| 100 | 1 | 0.045 | 0.057 | 1.26× |
| 100 | 5 | 0.205 | 0.283 | 1.38× |
| 100 | 10 | 0.446 | 0.599 | 1.34× |
| 1,000 | 1 | 0.052 | 0.054 | 1.04× |
| 1,000 | 5 | 0.226 | 0.276 | 1.22× |
| 1,000 | 10 | 0.460 | 0.599 | 1.30× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
