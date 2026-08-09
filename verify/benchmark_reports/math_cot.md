# MathCot benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.62M | 0.021 | 47.86M | 0.020 | 0.95× | 0.96× |
| 10,000 | 0.204 | 49.09M | 0.200 | 49.89M | 0.209 | 1.02× | 1.04× |
| 100,000 | 2.068 | 48.37M | 2.003 | 49.91M | 2.090 | 1.01× | 1.04× |
| 1,000,000 | 21.033 | 47.55M | 20.671 | 48.38M | 21.438 | 1.02× | 1.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.083 | 0.78× |
| 1 | 5 | 0.242 | 0.201 | 0.83× |
| 1 | 10 | 0.452 | 0.397 | 0.88× |
| 10 | 1 | 0.048 | 0.046 | 0.96× |
| 10 | 5 | 0.247 | 0.263 | 1.07× |
| 10 | 10 | 0.462 | 0.418 | 0.90× |
| 100 | 1 | 0.051 | 0.047 | 0.92× |
| 100 | 5 | 0.267 | 0.227 | 0.85× |
| 100 | 10 | 0.532 | 0.469 | 0.88× |
| 1,000 | 1 | 0.067 | 0.067 | 1.01× |
| 1,000 | 5 | 0.245 | 0.225 | 0.92× |
| 1,000 | 10 | 0.524 | 0.494 | 0.94× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.331 | 0.174 | 5.74M | nan | — | — |
| 100,000 | 10 | 1.247 | 1.078 | 9.28M | nan | — | — |
| 100,000 | 1,000 | 31.685 | 31.162 | 32.09M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 44.24M | 46.09M | 1.00× | 2.12M | 2.93M | 1.00× | — |
| 5 | 150.43M | 166.86M | 3.62× | 1.96M | 2.68M | 0.91× | — |
| 10 | 238.94M | 277.91M | 6.03× | 1.86M | 2.22M | 0.76× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
