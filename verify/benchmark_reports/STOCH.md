# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.85M | 0.015 | 66.59M | 0.054 | 3.41× | 3.61× |
| 10,000 | 0.148 | 67.59M | 0.143 | 70.07M | 0.172 | 1.17× | 1.21× |
| 100,000 | 1.609 | 62.14M | 1.490 | 67.10M | 1.325 | 0.82× | 0.89× |
| 1,000,000 | 18.552 | 53.90M | 16.853 | 59.34M | 13.575 | 0.73× | 0.81× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.131 | 0.191 | 1.46× |
| 1 | 5 | 0.322 | 0.584 | 1.81× |
| 1 | 10 | 0.505 | 1.155 | 2.29× |
| 10 | 1 | 0.067 | 0.131 | 1.95× |
| 10 | 5 | 0.255 | 0.550 | 2.16× |
| 10 | 10 | 0.558 | 1.104 | 1.98× |
| 100 | 1 | 0.059 | 0.109 | 1.83× |
| 100 | 5 | 0.283 | 0.584 | 2.06× |
| 100 | 10 | 0.610 | 1.175 | 1.93× |
| 1,000 | 1 | 0.070 | 0.121 | 1.74× |
| 1,000 | 5 | 0.309 | 0.669 | 2.16× |
| 1,000 | 10 | 0.598 | 1.304 | 2.18× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
