# VariablePeriodMovingAverage benchmark (`MAVP` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.848 | 1.18M | 0.806 | 1.24M | 0.110 | 0.13× | 0.14× |
| 10,000 | 8.089 | 1.24M | 7.832 | 1.28M | 0.743 | 0.09× | 0.09× |
| 100,000 | 79.222 | 1.26M | 81.150 | 1.23M | 7.404 | 0.09× | 0.09× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.142 | 0.130 | 0.92× |
| 1 | 5 | 0.541 | 0.515 | 0.95× |
| 1 | 10 | 0.738 | 1.009 | 1.37× |
| 10 | 1 | 0.082 | 0.105 | 1.28× |
| 10 | 5 | 0.348 | 0.485 | 1.39× |
| 10 | 10 | 0.771 | 1.025 | 1.33× |
| 100 | 1 | 0.159 | 0.106 | 0.67× |
| 100 | 5 | 0.345 | 0.521 | 1.51× |
| 100 | 10 | 0.795 | 1.098 | 1.38× |
| 1,000 | 1 | 0.907 | 0.182 | 0.20× |
| 1,000 | 5 | 1.113 | 0.891 | 0.80× |
| 1,000 | 10 | 2.120 | 1.928 | 0.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
