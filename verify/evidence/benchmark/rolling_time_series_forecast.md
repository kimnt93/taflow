# RollingTimeSeriesForecast benchmark (`TSF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.169 | 5.93M | 0.179 | 5.59M | 0.042 | 0.25× | 0.24× |
| 10,000 | 1.610 | 6.21M | 1.796 | 5.57M | 0.153 | 0.10× | 0.09× |
| 100,000 | 16.368 | 6.11M | 16.780 | 5.96M | 1.229 | 0.08× | 0.07× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.169 | 0.139 | 0.82× |
| 1 | 5 | 0.393 | 0.490 | 1.25× |
| 1 | 10 | 0.592 | 0.954 | 1.61× |
| 10 | 1 | 0.065 | 0.093 | 1.42× |
| 10 | 5 | 0.303 | 0.481 | 1.58× |
| 10 | 10 | 0.613 | 1.252 | 2.04× |
| 100 | 1 | 0.108 | 0.106 | 0.98× |
| 100 | 5 | 0.368 | 0.483 | 1.31× |
| 100 | 10 | 0.638 | 0.998 | 1.56× |
| 1,000 | 1 | 0.261 | 0.110 | 0.42× |
| 1,000 | 5 | 0.492 | 0.538 | 1.09× |
| 1,000 | 10 | 0.916 | 1.185 | 1.29× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
