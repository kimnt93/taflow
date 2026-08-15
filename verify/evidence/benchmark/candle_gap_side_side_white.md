# CandleGapSideSideWhite benchmark (`CDLGAPSIDESIDEWHITE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 86.77M | 0.008 | 121.65M | 0.046 | 4.00× | 5.61× |
| 10,000 | 0.122 | 81.65M | 0.120 | 83.17M | 0.222 | 1.81× | 1.84× |
| 100,000 | 1.278 | 78.26M | 1.230 | 81.27M | 1.964 | 1.54× | 1.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.133 | 1.03× |
| 1 | 5 | 0.282 | 0.474 | 1.68× |
| 1 | 10 | 0.412 | 0.929 | 2.25× |
| 10 | 1 | 0.042 | 0.093 | 2.20× |
| 10 | 5 | 0.191 | 0.418 | 2.19× |
| 10 | 10 | 0.389 | 0.918 | 2.36× |
| 100 | 1 | 0.051 | 0.103 | 2.03× |
| 100 | 5 | 0.204 | 0.444 | 2.18× |
| 100 | 10 | 0.404 | 0.946 | 2.34× |
| 1,000 | 1 | 0.058 | 0.114 | 1.96× |
| 1,000 | 5 | 0.191 | 0.633 | 3.32× |
| 1,000 | 10 | 0.428 | 1.098 | 2.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
