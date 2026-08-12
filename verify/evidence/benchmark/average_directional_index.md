# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 71.65M | 0.013 | 79.90M | 0.040 | 2.86× | 3.19× |
| 10,000 | 0.090 | 110.93M | 0.088 | 114.11M | 0.123 | 1.37× | 1.41× |
| 100,000 | 0.839 | 119.12M | 0.819 | 122.04M | 1.027 | 1.22× | 1.25× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.147 | 1.77× |
| 1 | 5 | 0.381 | 0.488 | 1.28× |
| 1 | 10 | 0.489 | 0.947 | 1.94× |
| 10 | 1 | 0.052 | 0.091 | 1.75× |
| 10 | 5 | 0.240 | 0.462 | 1.92× |
| 10 | 10 | 0.486 | 0.970 | 2.00× |
| 100 | 1 | 0.054 | 0.110 | 2.04× |
| 100 | 5 | 0.239 | 0.476 | 1.99× |
| 100 | 10 | 0.506 | 0.942 | 1.86× |
| 1,000 | 1 | 0.063 | 0.101 | 1.61× |
| 1,000 | 5 | 0.235 | 0.518 | 2.20× |
| 1,000 | 10 | 0.556 | 1.119 | 2.01× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
