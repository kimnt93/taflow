# HilbertTransformDominantCyclePeriod benchmark (`HT_DCPERIOD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.047 | 21.32M | 0.043 | 23.06M | 0.069 | 1.47× | 1.59× |
| 10,000 | 0.429 | 23.29M | 0.425 | 23.52M | 0.439 | 1.02× | 1.03× |
| 100,000 | 4.255 | 23.50M | 4.371 | 22.88M | 4.110 | 0.97× | 0.94× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.102 | 1.33× |
| 1 | 5 | 0.288 | 0.433 | 1.50× |
| 1 | 10 | 0.465 | 0.887 | 1.91× |
| 10 | 1 | 0.050 | 0.087 | 1.75× |
| 10 | 5 | 0.217 | 0.427 | 1.96× |
| 10 | 10 | 0.456 | 0.865 | 1.90× |
| 100 | 1 | 0.051 | 0.093 | 1.83× |
| 100 | 5 | 0.235 | 0.457 | 1.95× |
| 100 | 10 | 0.479 | 0.931 | 1.94× |
| 1,000 | 1 | 0.097 | 0.132 | 1.37× |
| 1,000 | 5 | 0.247 | 0.660 | 2.68× |
| 1,000 | 10 | 0.490 | 1.370 | 2.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
