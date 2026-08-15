# MathLn benchmark (`LN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 130.12M | 0.005 | 185.98M | 0.033 | 4.29× | 6.14× |
| 10,000 | 0.045 | 222.93M | 0.042 | 235.31M | 0.070 | 1.56× | 1.64× |
| 100,000 | 0.441 | 227.01M | 0.513 | 194.93M | 0.437 | 0.99× | 0.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.106 | 0.115 | 1.09× |
| 1 | 5 | 0.206 | 0.510 | 2.48× |
| 1 | 10 | 0.446 | 0.892 | 2.00× |
| 10 | 1 | 0.047 | 0.090 | 1.92× |
| 10 | 5 | 0.193 | 0.433 | 2.25× |
| 10 | 10 | 0.387 | 0.984 | 2.54× |
| 100 | 1 | 0.042 | 0.089 | 2.09× |
| 100 | 5 | 0.198 | 0.485 | 2.45× |
| 100 | 10 | 0.425 | 1.068 | 2.52× |
| 1,000 | 1 | 0.073 | 0.120 | 1.64× |
| 1,000 | 5 | 0.233 | 0.562 | 2.41× |
| 1,000 | 10 | 0.471 | 1.036 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
