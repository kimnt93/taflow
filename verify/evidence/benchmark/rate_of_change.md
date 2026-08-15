# RateOfChange benchmark (`ROC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.002 | 447.19M | 0.001 | 702.87M | 0.031 | 13.99× | 22.00× |
| 10,000 | 0.008 | 1.20G | 0.006 | 1.72G | 0.041 | 4.88× | 6.96× |
| 100,000 | 0.073 | 1.37G | 0.047 | 2.11G | 0.129 | 1.76× | 2.72× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.104 | 0.108 | 1.04× |
| 1 | 5 | 0.252 | 0.468 | 1.86× |
| 1 | 10 | 0.383 | 0.992 | 2.59× |
| 10 | 1 | 0.045 | 0.088 | 1.93× |
| 10 | 5 | 0.177 | 0.444 | 2.51× |
| 10 | 10 | 0.396 | 0.927 | 2.34× |
| 100 | 1 | 0.046 | 0.097 | 2.13× |
| 100 | 5 | 0.197 | 0.444 | 2.25× |
| 100 | 10 | 0.384 | 0.905 | 2.36× |
| 1,000 | 1 | 0.041 | 0.091 | 2.20× |
| 1,000 | 5 | 0.187 | 0.434 | 2.32× |
| 1,000 | 10 | 0.436 | 0.938 | 2.15× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
