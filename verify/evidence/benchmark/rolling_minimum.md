# RollingMinimum benchmark (`MIN` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 176.01M | 0.005 | 218.18M | 0.036 | 6.32× | 7.83× |
| 10,000 | 0.034 | 294.15M | 0.032 | 314.46M | 0.076 | 2.22× | 2.37× |
| 100,000 | 0.333 | 300.01M | 0.315 | 317.17M | 0.508 | 1.52× | 1.61× |
| 1,000,000 | 3.819 | 261.82M | 3.414 | 292.95M | 4.671 | 1.22× | 1.37× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.095 | 0.114 | 1.19× |
| 1 | 5 | 0.346 | 0.472 | 1.36× |
| 1 | 10 | 0.491 | 0.981 | 2.00× |
| 10 | 1 | 0.052 | 0.087 | 1.68× |
| 10 | 5 | 0.247 | 0.457 | 1.86× |
| 10 | 10 | 0.487 | 0.942 | 1.93× |
| 100 | 1 | 0.050 | 0.098 | 1.94× |
| 100 | 5 | 0.234 | 0.442 | 1.89× |
| 100 | 10 | 0.493 | 0.942 | 1.91× |
| 1,000 | 1 | 0.056 | 0.096 | 1.71× |
| 1,000 | 5 | 0.223 | 0.473 | 2.12× |
| 1,000 | 10 | 0.451 | 0.982 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
