# MathAbs benchmark (`numpy.abs` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.30M | 0.003 | 382.83M | 0.002 | 0.35× | 0.83× |
| 10,000 | 0.008 | 1.26G | 0.008 | 1.19G | 0.004 | 0.56× | 0.53× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.055 | 0.41× |
| 1 | 5 | 0.281 | 0.217 | 0.77× |
| 1 | 10 | 0.520 | 0.444 | 0.85× |
| 10 | 1 | 0.047 | 0.039 | 0.82× |
| 10 | 5 | 0.230 | 0.194 | 0.84× |
| 10 | 10 | 0.520 | 0.515 | 0.99× |
| 100 | 1 | 0.062 | 0.062 | 1.00× |
| 100 | 5 | 0.243 | 0.199 | 0.82× |
| 100 | 10 | 0.523 | 0.423 | 0.81× |
| 1,000 | 1 | 0.049 | 0.042 | 0.86× |
| 1,000 | 5 | 0.257 | 0.214 | 0.83× |
| 1,000 | 10 | 0.513 | 0.443 | 0.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
