# MathMultiply benchmark (`MULT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.030 | 33.88M | 0.022 | 46.18M | 0.031 | 1.05× | 1.42× |
| 10,000 | 0.193 | 51.77M | 0.151 | 66.23M | 0.034 | 0.18× | 0.22× |
| 100,000 | 1.401 | 71.40M | 1.392 | 71.82M | 0.076 | 0.05× | 0.05× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.111 | 0.89× |
| 1 | 5 | 0.409 | 0.446 | 1.09× |
| 1 | 10 | 0.603 | 0.916 | 1.52× |
| 10 | 1 | 0.064 | 0.086 | 1.34× |
| 10 | 5 | 0.301 | 0.442 | 1.47× |
| 10 | 10 | 0.602 | 0.940 | 1.56× |
| 100 | 1 | 0.077 | 0.093 | 1.20× |
| 100 | 5 | 0.321 | 0.431 | 1.34× |
| 100 | 10 | 0.599 | 0.905 | 1.51× |
| 1,000 | 1 | 0.077 | 0.086 | 1.11× |
| 1,000 | 5 | 0.298 | 0.437 | 1.47× |
| 1,000 | 10 | 0.619 | 0.926 | 1.50× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
