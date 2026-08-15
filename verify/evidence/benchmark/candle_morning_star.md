# CandleMorningStar benchmark (`CDLMORNINGSTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 153.93M | 0.004 | 282.57M | 0.038 | 5.91× | 10.85× |
| 10,000 | 0.069 | 145.68M | 0.058 | 172.63M | 0.125 | 1.82× | 2.16× |
| 100,000 | 0.933 | 107.14M | 0.849 | 117.74M | 0.846 | 0.91× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.119 | 0.121 | 1.02× |
| 1 | 5 | 0.189 | 0.510 | 2.70× |
| 1 | 10 | 0.415 | 0.955 | 2.30× |
| 10 | 1 | 0.043 | 0.088 | 2.05× |
| 10 | 5 | 0.180 | 0.461 | 2.56× |
| 10 | 10 | 0.366 | 0.987 | 2.69× |
| 100 | 1 | 0.046 | 0.092 | 2.01× |
| 100 | 5 | 0.181 | 0.450 | 2.48× |
| 100 | 10 | 0.383 | 0.966 | 2.52× |
| 1,000 | 1 | 0.067 | 0.109 | 1.62× |
| 1,000 | 5 | 0.192 | 0.497 | 2.59× |
| 1,000 | 10 | 0.441 | 1.026 | 2.33× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
