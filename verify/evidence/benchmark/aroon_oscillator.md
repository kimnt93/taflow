# AroonOscillator benchmark (`AROONOSC` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.44M | 0.009 | 116.11M | 0.040 | 4.25× | 4.60× |
| 10,000 | 0.121 | 82.90M | 0.111 | 90.01M | 0.143 | 1.18× | 1.29× |
| 100,000 | 1.185 | 84.42M | 1.136 | 88.02M | 1.085 | 0.92× | 0.95× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.096 | 0.118 | 1.24× |
| 1 | 5 | 0.265 | 0.459 | 1.73× |
| 1 | 10 | 0.415 | 1.004 | 2.42× |
| 10 | 1 | 0.050 | 0.089 | 1.78× |
| 10 | 5 | 0.196 | 0.451 | 2.30× |
| 10 | 10 | 0.390 | 1.034 | 2.65× |
| 100 | 1 | 0.055 | 0.108 | 1.96× |
| 100 | 5 | 0.208 | 0.505 | 2.43× |
| 100 | 10 | 0.415 | 0.974 | 2.34× |
| 1,000 | 1 | 0.055 | 0.103 | 1.86× |
| 1,000 | 5 | 0.227 | 0.556 | 2.45× |
| 1,000 | 10 | 0.410 | 1.091 | 2.66× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
