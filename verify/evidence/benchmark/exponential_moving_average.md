# ExponentialMovingAverage benchmark (`EMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 166.82M | 0.005 | 182.90M | 0.040 | 6.66× | 7.30× |
| 10,000 | 0.031 | 325.11M | 0.031 | 319.31M | 0.065 | 2.12× | 2.08× |
| 100,000 | 0.317 | 315.09M | 0.281 | 356.37M | 0.332 | 1.05× | 1.18× |
| 1,000,000 | 4.402 | 227.19M | 4.171 | 239.77M | 3.139 | 0.71× | 0.75× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.114 | 1.49× |
| 1 | 5 | 0.261 | 0.530 | 2.03× |
| 1 | 10 | 0.492 | 1.010 | 2.05× |
| 10 | 1 | 0.051 | 0.098 | 1.93× |
| 10 | 5 | 0.249 | 0.481 | 1.93× |
| 10 | 10 | 0.483 | 1.007 | 2.08× |
| 100 | 1 | 0.048 | 0.095 | 1.99× |
| 100 | 5 | 0.268 | 0.495 | 1.85× |
| 100 | 10 | 0.527 | 0.967 | 1.84× |
| 1,000 | 1 | 0.051 | 0.102 | 2.00× |
| 1,000 | 5 | 0.245 | 0.515 | 2.10× |
| 1,000 | 10 | 0.584 | 1.091 | 1.87× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
