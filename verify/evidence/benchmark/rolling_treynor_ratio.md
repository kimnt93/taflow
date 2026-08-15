# RollingTreynorRatio benchmark (`TreynorRatio` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.024 | 41.95M | 0.022 | 45.02M | 0.231 | 9.70× | 10.41× |
| 10,000 | 0.217 | 45.98M | 0.218 | 45.77M | 0.918 | 4.22× | 4.20× |
| 100,000 | 2.182 | 45.83M | 2.094 | 47.76M | 8.607 | 3.94× | 4.11× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.290 | 2.70× |
| 1 | 5 | 0.252 | 1.211 | 4.80× |
| 1 | 10 | 0.390 | 2.335 | 5.99× |
| 10 | 1 | 0.045 | 0.217 | 4.77× |
| 10 | 5 | 0.221 | 1.360 | 6.15× |
| 10 | 10 | 0.434 | 2.366 | 5.45× |
| 100 | 1 | 0.051 | 0.231 | 4.53× |
| 100 | 5 | 0.194 | 1.334 | 6.89× |
| 100 | 10 | 0.425 | 2.500 | 5.89× |
| 1,000 | 1 | 0.072 | 0.300 | 4.19× |
| 1,000 | 5 | 0.205 | 1.690 | 8.24× |
| 1,000 | 10 | 0.535 | 3.232 | 6.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
