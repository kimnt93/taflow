# MathAbs benchmark

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.003 | 298.16M | 0.002 | 438.99M | 0.002 | 0.53× | 0.78× |
| 10,000 | 0.007 | 1.38G | 0.005 | 2.17G | 0.003 | 0.48× | 0.76× |
| 100,000 | 0.057 | 1.74G | 0.033 | 3.01G | 0.032 | 0.56× | 0.96× |
| 1,000,000 | 0.869 | 1.15G | 0.701 | 1.43G | 0.649 | 0.75× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.121 | 0.089 | 0.74× |
| 1 | 5 | 0.279 | 0.232 | 0.83× |
| 1 | 10 | 0.541 | 0.431 | 0.80× |
| 10 | 1 | 0.050 | 0.044 | 0.89× |
| 10 | 5 | 0.203 | 0.199 | 0.98× |
| 10 | 10 | 0.516 | 0.433 | 0.84× |
| 100 | 1 | 0.050 | 0.042 | 0.84× |
| 100 | 5 | 0.212 | 0.180 | 0.85× |
| 100 | 10 | 0.470 | 0.419 | 0.89× |
| 1,000 | 1 | 0.047 | 0.046 | 0.97× |
| 1,000 | 5 | 0.235 | 0.208 | 0.88× |
| 1,000 | 10 | 0.549 | 0.425 | 0.77× |

## Warmed continuation

| Base | Chunk | API µs/call | Kernel µs/call | Kernel bars/s | TA-Lib full µs | vs full | vs tail |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 100,000 | 1 | 0.288 | 0.158 | 6.31M | nan | — | — |
| 100,000 | 10 | 0.997 | 0.533 | 18.75M | nan | — | — |
| 100,000 | 1,000 | 2.674 | 1.853 | 539.71M | nan | — | — |

## Independent-stream threads

| Threads | API vector/s | Kernel vector/s | Kernel vector scaling | API continue/s | Kernel continue/s | Kernel continue scaling | TA-Lib vector/s |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1 | 415.60M | 1.14G | 1.00× | 2.93M | 3.96M | 1.00× | — |
| 5 | 511.59M | 1.45G | 1.27× | 2.59M | 3.51M | 0.89× | — |
| 10 | 499.20M | 1.25G | 1.10× | 2.57M | 3.13M | 0.79× | — |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
