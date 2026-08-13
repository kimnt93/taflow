# VolumePriceTrend benchmark (`VolumePriceTrend` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.049 | 20.57M | 0.041 | 24.12M | 0.160 | 3.30× | 3.87× |
| 10,000 | 0.350 | 28.55M | 0.329 | 30.36M | 0.712 | 2.03× | 2.16× |
| 100,000 | 3.292 | 30.38M | 3.263 | 30.64M | 6.201 | 1.88× | 1.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.190 | 1.71× |
| 1 | 5 | 0.437 | 1.451 | 3.33× |
| 1 | 10 | 0.622 | 2.041 | 3.28× |
| 10 | 1 | 0.067 | 0.162 | 2.42× |
| 10 | 5 | 0.295 | 0.811 | 2.75× |
| 10 | 10 | 0.614 | 2.089 | 3.40× |
| 100 | 1 | 0.076 | 0.173 | 2.29× |
| 100 | 5 | 0.304 | 0.851 | 2.80× |
| 100 | 10 | 0.617 | 2.097 | 3.40× |
| 1,000 | 1 | 0.109 | 0.238 | 2.18× |
| 1,000 | 5 | 0.304 | 1.171 | 3.85× |
| 1,000 | 10 | 0.631 | 2.320 | 3.68× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
