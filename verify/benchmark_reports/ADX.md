# AverageDirectionalIndex benchmark (`ADX` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.012 | 81.17M | 0.012 | 86.36M | 0.043 | 3.48× | 3.71× |
| 10,000 | 0.113 | 88.50M | 0.086 | 115.65M | 0.128 | 1.13× | 1.48× |
| 100,000 | 0.847 | 118.05M | 0.914 | 109.40M | 1.022 | 1.21× | 1.12× |
| 1,000,000 | 9.859 | 101.43M | 9.079 | 110.15M | 10.161 | 1.03× | 1.12× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.094 | 0.135 | 1.44× |
| 1 | 5 | 0.272 | 0.525 | 1.93× |
| 1 | 10 | 0.553 | 1.219 | 2.20× |
| 10 | 1 | 0.059 | 0.101 | 1.73× |
| 10 | 5 | 0.253 | 0.544 | 2.15× |
| 10 | 10 | 0.583 | 1.140 | 1.96× |
| 100 | 1 | 0.059 | 0.128 | 2.16× |
| 100 | 5 | 0.293 | 0.590 | 2.02× |
| 100 | 10 | 0.554 | 1.110 | 2.00× |
| 1,000 | 1 | 0.071 | 0.118 | 1.66× |
| 1,000 | 5 | 0.310 | 0.634 | 2.04× |
| 1,000 | 10 | 0.629 | 1.246 | 1.98× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
