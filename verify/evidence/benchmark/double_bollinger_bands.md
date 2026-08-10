# DoubleBollingerBands benchmark (`DoubleBollinger` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.046 | 21.64M | 0.045 | 22.42M | 0.767 | 16.61× | 17.20× |
| 10,000 | 0.487 | 20.55M | 0.499 | 20.04M | 6.922 | 14.23× | 13.87× |
| 100,000 | 5.921 | 16.89M | 4.998 | 20.01M | 49.717 | 8.40× | 9.95× |
| 1,000,000 | 60.977 | 16.40M | 41.756 | 23.95M | 552.123 | 9.05× | 13.22× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.364 | 4.78× |
| 1 | 5 | 0.278 | 1.450 | 5.22× |
| 1 | 10 | 0.482 | 2.911 | 6.04× |
| 10 | 1 | 0.065 | 0.262 | 4.04× |
| 10 | 5 | 0.233 | 1.484 | 6.36× |
| 10 | 10 | 0.489 | 3.109 | 6.35× |
| 100 | 1 | 0.064 | 0.307 | 4.82× |
| 100 | 5 | 0.256 | 1.685 | 6.58× |
| 100 | 10 | 0.532 | 3.314 | 6.24× |
| 1,000 | 1 | 0.104 | 0.803 | 7.75× |
| 1,000 | 5 | 0.269 | 3.946 | 14.67× |
| 1,000 | 10 | 0.548 | 7.983 | 14.56× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
