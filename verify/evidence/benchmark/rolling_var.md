# RollingVariance benchmark (`VAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 145.67M | 0.006 | 165.92M | 0.036 | 5.28× | 6.01× |
| 10,000 | 0.039 | 253.72M | 0.040 | 249.75M | 0.059 | 1.51× | 1.48× |
| 100,000 | 0.411 | 243.11M | 0.377 | 265.39M | 0.267 | 0.65× | 0.71× |
| 1,000,000 | 4.184 | 239.03M | 3.651 | 273.93M | 2.498 | 0.60× | 0.68× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.172 | 1.44× |
| 1 | 5 | 0.273 | 0.484 | 1.78× |
| 1 | 10 | 0.448 | 1.016 | 2.27× |
| 10 | 1 | 0.065 | 0.116 | 1.80× |
| 10 | 5 | 0.259 | 0.645 | 2.49× |
| 10 | 10 | 0.482 | 0.960 | 1.99× |
| 100 | 1 | 0.047 | 0.096 | 2.03× |
| 100 | 5 | 0.243 | 0.545 | 2.24× |
| 100 | 10 | 0.491 | 0.946 | 1.93× |
| 1,000 | 1 | 0.052 | 0.110 | 2.12× |
| 1,000 | 5 | 0.245 | 0.467 | 1.91× |
| 1,000 | 10 | 0.565 | 1.061 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
