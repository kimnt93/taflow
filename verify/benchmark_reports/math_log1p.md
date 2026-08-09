# MathLog1p benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 89.55M | 0.010 | 99.93M | 0.009 | 0.81× | 0.91× |
| 10,000 | 0.082 | 122.48M | 0.077 | 130.11M | 0.079 | 0.96× | 1.02× |
| 100,000 | 0.828 | 120.70M | 0.752 | 132.94M | 0.802 | 0.97× | 1.07× |
| 1,000,000 | 7.865 | 127.14M | 7.414 | 134.89M | 7.378 | 0.94× | 1.00× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.155 | 0.091 | 0.59× |
| 1 | 5 | 0.267 | 0.224 | 0.84× |
| 1 | 10 | 0.461 | 0.378 | 0.82× |
| 10 | 1 | 0.047 | 0.040 | 0.84× |
| 10 | 5 | 0.228 | 0.187 | 0.82× |
| 10 | 10 | 0.453 | 0.370 | 0.82× |
| 100 | 1 | 0.044 | 0.041 | 0.94× |
| 100 | 5 | 0.222 | 0.184 | 0.83× |
| 100 | 10 | 0.468 | 0.376 | 0.80× |
| 1,000 | 1 | 0.052 | 0.048 | 0.91× |
| 1,000 | 5 | 0.213 | 0.199 | 0.93× |
| 1,000 | 10 | 0.494 | 0.432 | 0.87× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.327 | 0.172 | 5.80M | nan | — | — |
| 100,000 | 10 | 1.243 | 0.593 | 16.86M | nan | — | — |
| 100,000 | 1,000 | 14.734 | 9.219 | 108.47M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 118.96M | 122.01M | 1.00× | 2.62M | 3.54M | 1.00× | — |
| 5 | 314.29M | 390.27M | 3.20× | 2.06M | 2.50M | 0.71× | — |
| 10 | 380.57M | 553.72M | 4.54× | 2.14M | 2.49M | 0.70× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
