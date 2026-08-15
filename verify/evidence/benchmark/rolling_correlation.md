# RollingCorrelation benchmark (`CORREL` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 143.62M | 0.006 | 180.57M | 0.041 | 5.83× | 7.33× |
| 10,000 | 0.055 | 182.26M | 0.047 | 212.04M | 0.089 | 1.62× | 1.89× |
| 100,000 | 0.471 | 212.38M | 0.450 | 222.37M | 0.560 | 1.19× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.069 | 0.124 | 1.81× |
| 1 | 5 | 0.238 | 0.505 | 2.12× |
| 1 | 10 | 0.395 | 0.958 | 2.43× |
| 10 | 1 | 0.040 | 0.090 | 2.22× |
| 10 | 5 | 0.181 | 0.446 | 2.47× |
| 10 | 10 | 0.415 | 1.008 | 2.43× |
| 100 | 1 | 0.049 | 0.100 | 2.04× |
| 100 | 5 | 0.216 | 0.472 | 2.19× |
| 100 | 10 | 0.406 | 1.031 | 2.54× |
| 1,000 | 1 | 0.047 | 0.105 | 2.23× |
| 1,000 | 5 | 0.212 | 0.500 | 2.35× |
| 1,000 | 10 | 0.434 | 1.040 | 2.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
