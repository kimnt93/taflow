# CandleUpDownSideGapThreeMethods benchmark (`CDLXSIDEGAP3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 78.58M | 0.011 | 92.65M | 0.031 | 2.47× | 2.91× |
| 10,000 | 0.068 | 146.05M | 0.065 | 154.16M | 0.088 | 1.28× | 1.35× |
| 100,000 | 0.589 | 169.74M | 0.600 | 166.56M | 0.602 | 1.02× | 1.00× |
| 1,000,000 | 6.430 | 155.51M | 5.962 | 167.74M | 5.802 | 0.90× | 0.97× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.125 | 1.13× |
| 1 | 5 | 0.455 | 0.589 | 1.30× |
| 1 | 10 | 0.519 | 0.939 | 1.81× |
| 10 | 1 | 0.052 | 0.093 | 1.79× |
| 10 | 5 | 0.245 | 0.433 | 1.76× |
| 10 | 10 | 0.485 | 0.933 | 1.92× |
| 100 | 1 | 0.057 | 0.093 | 1.64× |
| 100 | 5 | 0.252 | 0.459 | 1.82× |
| 100 | 10 | 0.519 | 0.947 | 1.82× |
| 1,000 | 1 | 0.060 | 0.098 | 1.62× |
| 1,000 | 5 | 0.252 | 0.469 | 1.87× |
| 1,000 | 10 | 0.553 | 1.002 | 1.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
