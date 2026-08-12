# CandleTakuri benchmark (`CDLTAKURI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 56.82M | 0.015 | 67.66M | 0.039 | 2.24× | 2.67× |
| 10,000 | 0.163 | 61.41M | 0.115 | 87.13M | 0.107 | 0.66× | 0.93× |
| 100,000 | 1.186 | 84.29M | 1.377 | 72.64M | 0.839 | 0.71× | 0.61× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.113 | 1.01× |
| 1 | 5 | 0.344 | 0.515 | 1.50× |
| 1 | 10 | 0.516 | 0.899 | 1.74× |
| 10 | 1 | 0.052 | 0.096 | 1.84× |
| 10 | 5 | 0.239 | 0.419 | 1.75× |
| 10 | 10 | 0.595 | 0.936 | 1.57× |
| 100 | 1 | 0.055 | 0.088 | 1.60× |
| 100 | 5 | 0.247 | 0.439 | 1.78× |
| 100 | 10 | 0.548 | 1.032 | 1.88× |
| 1,000 | 1 | 0.076 | 0.101 | 1.33× |
| 1,000 | 5 | 0.295 | 0.496 | 1.68× |
| 1,000 | 10 | 0.564 | 1.099 | 1.95× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
