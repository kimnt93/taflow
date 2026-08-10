# NegativeVolumeIndex benchmark (`NVI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.008 | 127.34M | 0.006 | 156.66M | 0.178 | 22.60× | 27.81× |
| 10,000 | 0.058 | 171.32M | 0.054 | 185.50M | 0.750 | 12.85× | 13.91× |
| 100,000 | 0.552 | 181.27M | 0.636 | 157.19M | 7.277 | 13.19× | 11.44× |
| 1,000,000 | 5.961 | 167.75M | 5.311 | 188.28M | 65.540 | 10.99× | 12.34× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.093 | 0.238 | 2.58× |
| 1 | 5 | 0.333 | 1.260 | 3.78× |
| 1 | 10 | 0.473 | 2.184 | 4.62× |
| 10 | 1 | 0.049 | 0.199 | 4.04× |
| 10 | 5 | 0.233 | 1.285 | 5.51× |
| 10 | 10 | 0.485 | 2.248 | 4.64× |
| 100 | 1 | 0.057 | 0.203 | 3.53× |
| 100 | 5 | 0.260 | 1.334 | 5.13× |
| 100 | 10 | 0.516 | 2.330 | 4.51× |
| 1,000 | 1 | 0.062 | 0.264 | 4.30× |
| 1,000 | 5 | 0.257 | 1.613 | 6.26× |
| 1,000 | 10 | 0.558 | 2.914 | 5.22× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
