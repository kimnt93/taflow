# RollingMax benchmark (`MAX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 149.87M | 0.006 | 177.26M | 0.041 | 6.09× | 7.20× |
| 10,000 | 0.037 | 269.28M | 0.034 | 290.89M | 0.084 | 2.25× | 2.43× |
| 100,000 | 0.373 | 268.16M | 0.349 | 286.36M | 0.541 | 1.45× | 1.55× |
| 1,000,000 | 4.426 | 225.96M | 3.954 | 252.90M | 5.102 | 1.15× | 1.29× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.080 | 0.147 | 1.83× |
| 1 | 5 | 0.259 | 0.471 | 1.81× |
| 1 | 10 | 0.445 | 0.976 | 2.19× |
| 10 | 1 | 0.052 | 0.099 | 1.89× |
| 10 | 5 | 0.256 | 0.497 | 1.94× |
| 10 | 10 | 0.484 | 1.007 | 2.08× |
| 100 | 1 | 0.063 | 0.099 | 1.58× |
| 100 | 5 | 0.251 | 0.509 | 2.03× |
| 100 | 10 | 0.517 | 1.006 | 1.95× |
| 1,000 | 1 | 0.054 | 0.102 | 1.89× |
| 1,000 | 5 | 0.263 | 0.502 | 1.91× |
| 1,000 | 10 | 0.540 | 1.081 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
