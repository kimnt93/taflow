# StochasticOscillator benchmark (`STOCH` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 66.35M | 0.012 | 81.29M | 0.054 | 3.59× | 4.40× |
| 10,000 | 0.110 | 91.11M | 0.103 | 97.45M | 0.171 | 1.56× | 1.67× |
| 100,000 | 1.019 | 98.10M | 0.981 | 101.90M | 1.216 | 1.19× | 1.24× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.184 | 0.147 | 0.80× |
| 1 | 5 | 0.227 | 0.536 | 2.36× |
| 1 | 10 | 0.441 | 1.096 | 2.48× |
| 10 | 1 | 0.050 | 0.156 | 3.11× |
| 10 | 5 | 0.249 | 1.212 | 4.87× |
| 10 | 10 | 0.446 | 1.184 | 2.65× |
| 100 | 1 | 0.050 | 0.109 | 2.18× |
| 100 | 5 | 0.220 | 0.623 | 2.83× |
| 100 | 10 | 0.469 | 1.118 | 2.38× |
| 1,000 | 1 | 0.058 | 0.125 | 2.15× |
| 1,000 | 5 | 0.209 | 0.611 | 2.93× |
| 1,000 | 10 | 0.509 | 1.251 | 2.46× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
