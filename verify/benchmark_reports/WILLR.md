# WilliamsPercentR benchmark (`WILLR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 83.78M | 0.010 | 96.63M | 0.037 | 3.06× | 3.53× |
| 10,000 | 0.096 | 103.87M | 0.100 | 99.71M | 0.156 | 1.62× | 1.55× |
| 100,000 | 0.923 | 108.32M | 0.889 | 112.49M | 0.867 | 0.94× | 0.98× |
| 1,000,000 | 10.293 | 97.15M | 10.112 | 98.89M | 8.533 | 0.83× | 0.84× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.148 | 1.27× |
| 1 | 5 | 0.355 | 0.486 | 1.37× |
| 1 | 10 | 0.515 | 1.022 | 1.99× |
| 10 | 1 | 0.056 | 0.100 | 1.79× |
| 10 | 5 | 0.225 | 0.431 | 1.91× |
| 10 | 10 | 0.495 | 0.998 | 2.01× |
| 100 | 1 | 0.060 | 0.101 | 1.70× |
| 100 | 5 | 0.256 | 0.471 | 1.84× |
| 100 | 10 | 0.526 | 1.016 | 1.93× |
| 1,000 | 1 | 0.064 | 0.108 | 1.69× |
| 1,000 | 5 | 0.299 | 0.557 | 1.86× |
| 1,000 | 10 | 0.554 | 1.106 | 2.00× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
