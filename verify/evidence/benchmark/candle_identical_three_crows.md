# CandleIdenticalThreeCrows benchmark (`CDLIDENTICAL3CROWS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.156 | 6.41M | 0.144 | 6.95M | 0.036 | 0.23× | 0.25× |
| 10,000 | 1.353 | 7.39M | 1.328 | 7.53M | 0.114 | 0.08× | 0.09× |
| 100,000 | 13.452 | 7.43M | 13.071 | 7.65M | 0.874 | 0.06× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.145 | 0.127 | 0.88× |
| 1 | 5 | 0.397 | 0.504 | 1.27× |
| 1 | 10 | 0.639 | 0.906 | 1.42× |
| 10 | 1 | 0.069 | 0.089 | 1.30× |
| 10 | 5 | 0.302 | 0.417 | 1.38× |
| 10 | 10 | 0.658 | 0.979 | 1.49× |
| 100 | 1 | 0.095 | 0.096 | 1.01× |
| 100 | 5 | 0.373 | 0.462 | 1.24× |
| 100 | 10 | 0.661 | 0.926 | 1.40× |
| 1,000 | 1 | 0.206 | 0.108 | 0.52× |
| 1,000 | 5 | 0.371 | 0.493 | 1.33× |
| 1,000 | 10 | 0.758 | 1.045 | 1.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
