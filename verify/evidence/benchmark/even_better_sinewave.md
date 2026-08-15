# EvenBetterSinewave benchmark (`ebsw` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 147.02M | 0.006 | 168.35M | 11.557 | 1699.14× | 1945.73× |
| 10,000 | 0.054 | 184.75M | 0.053 | 188.99M | 118.667 | 2192.33× | 2242.63× |
| 100,000 | 0.548 | 182.59M | 0.511 | 195.57M | 1179.288 | 2153.22× | 2306.35× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.159 | 0.198 | 1.24× |
| 1 | 5 | 0.294 | 0.769 | 2.62× |
| 1 | 10 | 0.408 | 1.592 | 3.90× |
| 10 | 1 | 0.047 | 0.150 | 3.21× |
| 10 | 5 | 0.202 | 0.743 | 3.67× |
| 10 | 10 | 0.403 | 1.664 | 4.13× |
| 100 | 1 | 0.046 | 0.977 | 21.01× |
| 100 | 5 | 0.190 | 4.901 | 25.79× |
| 100 | 10 | 0.389 | 9.968 | 25.66× |
| 1,000 | 1 | 0.052 | 11.806 | 227.98× |
| 1,000 | 5 | 0.314 | 64.970 | 206.71× |
| 1,000 | 10 | 0.672 | 136.626 | 203.24× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
