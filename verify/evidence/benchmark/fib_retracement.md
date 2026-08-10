# FibonacciRetracement benchmark (`rolling Fibonacci levels` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.031 | 32.11M | 0.025 | 39.77M | 10.924 | 350.78× | 434.47× |
| 10,000 | 0.330 | 30.34M | 0.281 | 35.61M | 107.785 | 327.05× | 383.86× |
| 100,000 | 3.154 | 31.70M | 2.673 | 37.41M | 1071.243 | 339.61× | 400.79× |
| 1,000,000 | 60.498 | 16.53M | 28.161 | 35.51M | 10490.575 | 173.40× | 372.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.122 | 0.162 | 1.33× |
| 1 | 5 | 0.332 | 0.595 | 1.80× |
| 1 | 10 | 0.487 | 1.168 | 2.40× |
| 10 | 1 | 0.052 | 0.223 | 4.26× |
| 10 | 5 | 0.261 | 1.296 | 4.96× |
| 10 | 10 | 0.575 | 2.330 | 4.05× |
| 100 | 1 | 0.051 | 1.206 | 23.80× |
| 100 | 5 | 0.253 | 6.072 | 23.95× |
| 100 | 10 | 0.542 | 13.262 | 24.46× |
| 1,000 | 1 | 0.090 | 11.044 | 123.19× |
| 1,000 | 5 | 0.457 | 68.380 | 149.49× |
| 1,000 | 10 | 0.953 | 118.563 | 124.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
