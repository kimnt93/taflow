# RollingQuantile benchmark (`RollingQuantile` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.044 | 22.85M | 0.043 | 23.13M | 0.339 | 7.75× | 7.85× |
| 10,000 | 0.557 | 17.97M | 0.464 | 21.56M | 1.699 | 3.05× | 3.66× |
| 100,000 | 4.692 | 21.31M | 4.879 | 20.50M | 15.695 | 3.34× | 3.22× |
| 1,000,000 | 47.031 | 21.26M | 48.182 | 20.75M | 166.265 | 3.54× | 3.45× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.117 | 0.342 | 2.93× |
| 1 | 5 | 0.330 | 1.336 | 4.05× |
| 1 | 10 | 0.492 | 2.679 | 5.45× |
| 10 | 1 | 0.052 | 0.240 | 4.62× |
| 10 | 5 | 0.217 | 7.364 | 33.96× |
| 10 | 10 | 0.526 | 2.577 | 4.90× |
| 100 | 1 | 0.053 | 0.251 | 4.71× |
| 100 | 5 | 0.231 | 1.460 | 6.33× |
| 100 | 10 | 0.511 | 2.862 | 5.60× |
| 1,000 | 1 | 0.110 | 0.415 | 3.78× |
| 1,000 | 5 | 0.244 | 2.235 | 9.17× |
| 1,000 | 10 | 0.568 | 4.386 | 7.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
