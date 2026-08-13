# OpeningRange benchmark (`anchored opening range` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.066 | 15.15M | 0.056 | 17.93M | 0.538 | 8.14× | 9.64× |
| 10,000 | 0.434 | 23.06M | 0.429 | 23.30M | 5.019 | 11.57× | 11.69× |
| 100,000 | 4.222 | 23.69M | 4.052 | 24.68M | 49.372 | 11.69× | 12.19× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.110 | 0.159 | 1.45× |
| 1 | 5 | 0.402 | 0.461 | 1.15× |
| 1 | 10 | 0.731 | 0.924 | 1.26× |
| 10 | 1 | 0.082 | 0.101 | 1.23× |
| 10 | 5 | 0.314 | 0.463 | 1.47× |
| 10 | 10 | 0.668 | 0.910 | 1.36× |
| 100 | 1 | 0.074 | 0.148 | 2.01× |
| 100 | 5 | 0.343 | 0.713 | 2.08× |
| 100 | 10 | 0.724 | 1.435 | 1.98× |
| 1,000 | 1 | 0.118 | 0.617 | 5.24× |
| 1,000 | 5 | 0.552 | 3.101 | 5.62× |
| 1,000 | 10 | 1.130 | 6.160 | 5.45× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
