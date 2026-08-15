# DoubleBollingerBands benchmark (`DoubleBollinger` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.042 | 23.53M | 0.046 | 21.90M | 0.618 | 14.54× | 13.53× |
| 10,000 | 0.396 | 25.22M | 0.374 | 26.74M | 4.411 | 11.13× | 11.79× |
| 100,000 | 4.148 | 24.11M | 3.923 | 25.49M | 50.548 | 12.19× | 12.89× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.306 | 3.06× |
| 1 | 5 | 0.299 | 1.351 | 4.53× |
| 1 | 10 | 0.458 | 2.825 | 6.17× |
| 10 | 1 | 0.044 | 0.261 | 5.90× |
| 10 | 5 | 0.214 | 1.418 | 6.61× |
| 10 | 10 | 0.434 | 2.868 | 6.61× |
| 100 | 1 | 0.060 | 0.310 | 5.19× |
| 100 | 5 | 0.199 | 1.558 | 7.84× |
| 100 | 10 | 0.440 | 3.477 | 7.90× |
| 1,000 | 1 | 0.091 | 0.809 | 8.93× |
| 1,000 | 5 | 0.211 | 3.931 | 18.59× |
| 1,000 | 10 | 0.447 | 7.955 | 17.80× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
