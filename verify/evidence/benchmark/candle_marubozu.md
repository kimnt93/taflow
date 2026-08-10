# CandleMarubozu benchmark (`CDLMARUBOZU` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.023 | 43.82M | 0.028 | 36.28M | 0.040 | 1.76× | 1.45× |
| 10,000 | 0.173 | 57.67M | 0.168 | 59.64M | 0.154 | 0.89× | 0.92× |
| 100,000 | 1.641 | 60.96M | 1.667 | 59.98M | 1.246 | 0.76× | 0.75× |
| 1,000,000 | 16.496 | 60.62M | 15.822 | 63.20M | 11.945 | 0.72× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.169 | 1.44× |
| 1 | 5 | 0.408 | 0.646 | 1.58× |
| 1 | 10 | 0.668 | 1.144 | 1.71× |
| 10 | 1 | 0.065 | 0.109 | 1.68× |
| 10 | 5 | 0.355 | 0.575 | 1.62× |
| 10 | 10 | 0.655 | 1.123 | 1.71× |
| 100 | 1 | 0.083 | 0.123 | 1.49× |
| 100 | 5 | 0.382 | 0.588 | 1.54× |
| 100 | 10 | 0.697 | 1.140 | 1.64× |
| 1,000 | 1 | 0.089 | 0.127 | 1.42× |
| 1,000 | 5 | 0.386 | 0.667 | 1.73× |
| 1,000 | 10 | 0.714 | 1.275 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
