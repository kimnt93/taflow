# AverageTrueRangeBands benchmark (`AtrBands` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.35M | 0.015 | 65.99M | 0.619 | 34.27× | 40.85× |
| 10,000 | 0.113 | 88.86M | 0.116 | 86.13M | 4.867 | 43.24× | 41.92× |
| 100,000 | 1.146 | 87.24M | 0.991 | 100.92M | 47.744 | 41.65× | 48.18× |
| 1,000,000 | 12.869 | 77.71M | 11.661 | 85.76M | 540.323 | 41.99× | 46.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.114 | 0.355 | 3.12× |
| 1 | 5 | 0.343 | 1.495 | 4.35× |
| 1 | 10 | 0.536 | 3.289 | 6.14× |
| 10 | 1 | 0.056 | 0.266 | 4.74× |
| 10 | 5 | 0.241 | 1.457 | 6.04× |
| 10 | 10 | 0.618 | 3.141 | 5.09× |
| 100 | 1 | 0.056 | 0.314 | 5.63× |
| 100 | 5 | 0.325 | 1.836 | 5.65× |
| 100 | 10 | 0.553 | 3.456 | 6.25× |
| 1,000 | 1 | 0.074 | 0.907 | 12.19× |
| 1,000 | 5 | 0.376 | 4.098 | 10.89× |
| 1,000 | 10 | 0.647 | 8.797 | 13.60× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
