# RollingKendallRankCorrelation benchmark (`KendallTau` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.770 | 1.30M | 0.757 | 1.32M | 0.923 | 1.20× | 1.22× |
| 10,000 | 7.455 | 1.34M | 7.574 | 1.32M | 7.228 | 0.97× | 0.95× |
| 100,000 | 76.737 | 1.30M | 75.233 | 1.33M | 68.479 | 0.89× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.085 | 0.270 | 3.16× |
| 1 | 5 | 0.289 | 1.232 | 4.26× |
| 1 | 10 | 0.412 | 2.299 | 5.58× |
| 10 | 1 | 0.049 | 0.210 | 4.27× |
| 10 | 5 | 0.197 | 1.209 | 6.12× |
| 10 | 10 | 0.453 | 2.270 | 5.02× |
| 100 | 1 | 0.108 | 0.267 | 2.48× |
| 100 | 5 | 0.215 | 1.556 | 7.25× |
| 100 | 10 | 0.472 | 2.802 | 5.94× |
| 1,000 | 1 | 0.867 | 0.994 | 1.15× |
| 1,000 | 5 | 1.022 | 4.908 | 4.80× |
| 1,000 | 10 | 1.379 | 9.709 | 7.04× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
