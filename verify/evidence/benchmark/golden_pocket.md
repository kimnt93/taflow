# GoldenPocket benchmark (`GoldenPocket` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.17M | 0.015 | 65.17M | 0.460 | 25.85× | 29.99× |
| 10,000 | 0.132 | 75.65M | 0.126 | 79.21M | 3.694 | 27.95× | 29.26× |
| 100,000 | 1.343 | 74.46M | 1.310 | 76.31M | 38.993 | 29.03× | 29.75× |
| 1,000,000 | 14.250 | 70.18M | 12.536 | 79.77M | 432.284 | 30.34× | 34.48× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.208 | 2.61× |
| 1 | 5 | 0.280 | 0.882 | 3.15× |
| 1 | 10 | 0.482 | 1.883 | 3.91× |
| 10 | 1 | 0.055 | 0.174 | 3.18× |
| 10 | 5 | 0.248 | 0.843 | 3.40× |
| 10 | 10 | 0.481 | 1.864 | 3.87× |
| 100 | 1 | 0.063 | 0.209 | 3.33× |
| 100 | 5 | 0.252 | 1.046 | 4.15× |
| 100 | 10 | 0.510 | 2.273 | 4.46× |
| 1,000 | 1 | 0.077 | 0.640 | 8.30× |
| 1,000 | 5 | 0.254 | 3.084 | 12.15× |
| 1,000 | 10 | 0.529 | 7.420 | 14.03× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
