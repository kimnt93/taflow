# DoubleExponentialMovingAverage benchmark (`DEMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 196.97M | 0.004 | 237.24M | 0.037 | 7.35× | 8.85× |
| 10,000 | 0.035 | 284.14M | 0.033 | 301.14M | 0.095 | 2.69× | 2.85× |
| 100,000 | 0.353 | 283.65M | 0.330 | 302.88M | 0.667 | 1.89× | 2.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.098 | 0.102 | 1.03× |
| 1 | 5 | 0.273 | 0.527 | 1.93× |
| 1 | 10 | 0.394 | 0.969 | 2.46× |
| 10 | 1 | 0.041 | 0.088 | 2.15× |
| 10 | 5 | 0.188 | 0.440 | 2.34× |
| 10 | 10 | 0.504 | 0.977 | 1.94× |
| 100 | 1 | 0.039 | 0.088 | 2.23× |
| 100 | 5 | 0.180 | 0.433 | 2.40× |
| 100 | 10 | 0.438 | 0.993 | 2.27× |
| 1,000 | 1 | 0.049 | 0.112 | 2.30× |
| 1,000 | 5 | 0.193 | 0.505 | 2.62× |
| 1,000 | 10 | 0.439 | 0.965 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
