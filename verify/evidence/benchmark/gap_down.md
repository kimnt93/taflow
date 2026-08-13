# GapDown benchmark (`gap down relation` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.045 | 22.19M | 0.039 | 25.39M | 0.022 | 0.49× | 0.56× |
| 10,000 | 0.331 | 30.23M | 0.318 | 31.45M | 0.040 | 0.12× | 0.13× |
| 100,000 | 3.126 | 31.99M | 3.138 | 31.87M | 0.221 | 0.07× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.125 | 0.109 | 0.87× |
| 1 | 5 | 0.344 | 0.385 | 1.12× |
| 1 | 10 | 0.593 | 0.734 | 1.24× |
| 10 | 1 | 0.067 | 0.070 | 1.05× |
| 10 | 5 | 0.280 | 0.339 | 1.21× |
| 10 | 10 | 0.605 | 0.733 | 1.21× |
| 100 | 1 | 0.070 | 0.069 | 0.99× |
| 100 | 5 | 0.296 | 0.357 | 1.21× |
| 100 | 10 | 0.614 | 0.740 | 1.20× |
| 1,000 | 1 | 0.099 | 0.076 | 0.77× |
| 1,000 | 5 | 0.280 | 0.480 | 1.71× |
| 1,000 | 10 | 0.613 | 1.113 | 1.82× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
