# WeightedMovingAverage benchmark (`WMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 165.19M | 0.005 | 191.09M | 0.039 | 6.40× | 7.40× |
| 10,000 | 0.036 | 275.01M | 0.034 | 294.69M | 0.050 | 1.37× | 1.47× |
| 100,000 | 0.340 | 294.26M | 0.317 | 315.88M | 0.226 | 0.66× | 0.71× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.140 | 1.62× |
| 1 | 5 | 0.353 | 0.494 | 1.40× |
| 1 | 10 | 0.540 | 0.945 | 1.75× |
| 10 | 1 | 0.046 | 0.100 | 2.18× |
| 10 | 5 | 0.213 | 0.440 | 2.06× |
| 10 | 10 | 0.505 | 1.034 | 2.05× |
| 100 | 1 | 0.051 | 0.106 | 2.05× |
| 100 | 5 | 0.227 | 0.465 | 2.05× |
| 100 | 10 | 0.493 | 0.938 | 1.90× |
| 1,000 | 1 | 0.063 | 0.108 | 1.72× |
| 1,000 | 5 | 0.272 | 0.501 | 1.84× |
| 1,000 | 10 | 0.478 | 0.949 | 1.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
