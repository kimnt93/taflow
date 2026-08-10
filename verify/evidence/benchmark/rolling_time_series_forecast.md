# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.018 | 55.72M | 0.017 | 58.29M | 0.046 | 2.57× | 2.69× |
| 10,000 | 0.146 | 68.59M | 0.142 | 70.60M | 0.177 | 1.21× | 1.25× |
| 100,000 | 1.499 | 66.70M | 1.370 | 72.97M | 1.425 | 0.95× | 1.04× |
| 1,000,000 | 16.756 | 59.68M | 14.570 | 68.64M | 14.906 | 0.89× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.086 | 0.120 | 1.39× |
| 1 | 5 | 0.404 | 0.533 | 1.32× |
| 1 | 10 | 0.480 | 0.987 | 2.06× |
| 10 | 1 | 0.052 | 0.092 | 1.77× |
| 10 | 5 | 0.223 | 0.441 | 1.98× |
| 10 | 10 | 0.556 | 1.062 | 1.91× |
| 100 | 1 | 0.051 | 0.098 | 1.91× |
| 100 | 5 | 0.229 | 0.450 | 1.97× |
| 100 | 10 | 0.505 | 1.117 | 2.21× |
| 1,000 | 1 | 0.072 | 0.109 | 1.51× |
| 1,000 | 5 | 0.225 | 0.509 | 2.27× |
| 1,000 | 10 | 0.512 | 1.187 | 2.32× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
