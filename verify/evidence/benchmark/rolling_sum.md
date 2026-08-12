# RollingSum benchmark (`SUM` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 169.10M | 0.005 | 201.83M | 0.030 | 5.07× | 6.05× |
| 10,000 | 0.035 | 284.00M | 0.031 | 318.14M | 0.047 | 1.33× | 1.49× |
| 100,000 | 0.314 | 318.39M | 0.290 | 345.32M | 0.217 | 0.69× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.084 | 0.107 | 1.28× |
| 1 | 5 | 0.277 | 0.464 | 1.68× |
| 1 | 10 | 0.458 | 0.934 | 2.04× |
| 10 | 1 | 0.047 | 0.086 | 1.81× |
| 10 | 5 | 0.228 | 0.437 | 1.91× |
| 10 | 10 | 0.449 | 0.941 | 2.10× |
| 100 | 1 | 0.047 | 0.090 | 1.92× |
| 100 | 5 | 0.212 | 0.422 | 1.99× |
| 100 | 10 | 0.457 | 0.923 | 2.02× |
| 1,000 | 1 | 0.051 | 0.103 | 2.02× |
| 1,000 | 5 | 0.243 | 0.439 | 1.80× |
| 1,000 | 10 | 0.489 | 0.933 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
