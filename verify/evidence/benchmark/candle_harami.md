# CandleHarami benchmark (`CDLHARAMI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.66M | 0.016 | 60.87M | 0.034 | 1.60× | 2.04× |
| 10,000 | 0.137 | 73.10M | 0.145 | 69.14M | 0.133 | 0.97× | 0.92× |
| 100,000 | 1.383 | 72.32M | 1.309 | 76.38M | 1.148 | 0.83× | 0.88× |
| 1,000,000 | 14.268 | 70.09M | 12.999 | 76.93M | 10.980 | 0.77× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.115 | 1.44× |
| 1 | 5 | 0.396 | 0.456 | 1.15× |
| 1 | 10 | 0.535 | 0.898 | 1.68× |
| 10 | 1 | 0.055 | 0.089 | 1.61× |
| 10 | 5 | 0.264 | 0.426 | 1.61× |
| 10 | 10 | 0.537 | 0.895 | 1.67× |
| 100 | 1 | 0.056 | 0.090 | 1.60× |
| 100 | 5 | 0.246 | 0.413 | 1.68× |
| 100 | 10 | 0.507 | 0.869 | 1.71× |
| 1,000 | 1 | 0.064 | 0.103 | 1.61× |
| 1,000 | 5 | 0.248 | 0.485 | 1.96× |
| 1,000 | 10 | 0.568 | 1.017 | 1.79× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
