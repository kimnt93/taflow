# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.55M | 0.014 | 69.66M | 0.048 | 3.27× | 3.38× |
| 10,000 | 0.143 | 69.86M | 0.128 | 78.27M | 0.159 | 1.11× | 1.24× |
| 100,000 | 1.329 | 75.27M | 1.283 | 77.97M | 1.358 | 1.02× | 1.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.124 | 0.80× |
| 1 | 5 | 0.221 | 0.476 | 2.15× |
| 1 | 10 | 0.403 | 0.932 | 2.31× |
| 10 | 1 | 0.047 | 0.089 | 1.89× |
| 10 | 5 | 0.184 | 0.425 | 2.31× |
| 10 | 10 | 0.435 | 0.948 | 2.18× |
| 100 | 1 | 0.047 | 0.090 | 1.91× |
| 100 | 5 | 0.189 | 0.445 | 2.35× |
| 100 | 10 | 0.405 | 0.987 | 2.44× |
| 1,000 | 1 | 0.057 | 0.109 | 1.92× |
| 1,000 | 5 | 0.199 | 0.532 | 2.67× |
| 1,000 | 10 | 0.454 | 1.082 | 2.38× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
