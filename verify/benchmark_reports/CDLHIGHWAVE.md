# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 101.16M | 0.008 | 129.15M | 0.034 | 3.41× | 4.35× |
| 10,000 | 0.115 | 86.92M | 0.111 | 89.85M | 0.159 | 1.38× | 1.43× |
| 100,000 | 1.208 | 82.76M | 1.165 | 85.87M | 1.332 | 1.10× | 1.14× |
| 1,000,000 | 11.878 | 84.19M | 11.665 | 85.73M | 13.483 | 1.14× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.129 | 0.145 | 1.12× |
| 1 | 5 | 0.326 | 0.522 | 1.60× |
| 1 | 10 | 0.558 | 0.960 | 1.72× |
| 10 | 1 | 0.054 | 0.113 | 2.08× |
| 10 | 5 | 0.254 | 0.447 | 1.76× |
| 10 | 10 | 0.513 | 0.894 | 1.74× |
| 100 | 1 | 0.054 | 0.100 | 1.84× |
| 100 | 5 | 0.268 | 0.446 | 1.66× |
| 100 | 10 | 0.565 | 0.965 | 1.71× |
| 1,000 | 1 | 0.066 | 0.103 | 1.56× |
| 1,000 | 5 | 0.267 | 0.519 | 1.95× |
| 1,000 | 10 | 0.565 | 1.109 | 1.96× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
