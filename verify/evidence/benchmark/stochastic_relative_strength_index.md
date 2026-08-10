# StochasticRelativeStrengthIndex benchmark (`STOCHRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.08M | 0.022 | 45.84M | 0.051 | 2.30× | 2.34× |
| 10,000 | 0.230 | 43.53M | 0.225 | 44.50M | 0.198 | 0.86× | 0.88× |
| 100,000 | 2.413 | 41.44M | 2.207 | 45.31M | 1.579 | 0.65× | 0.72× |
| 1,000,000 | 25.340 | 39.46M | 23.387 | 42.76M | 15.141 | 0.60× | 0.65× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.115 | 0.130 | 1.12× |
| 1 | 5 | 0.305 | 0.529 | 1.74× |
| 1 | 10 | 0.473 | 1.039 | 2.20× |
| 10 | 1 | 0.053 | 0.102 | 1.92× |
| 10 | 5 | 0.220 | 0.505 | 2.30× |
| 10 | 10 | 0.467 | 1.039 | 2.23× |
| 100 | 1 | 0.057 | 0.106 | 1.87× |
| 100 | 5 | 0.247 | 0.516 | 2.09× |
| 100 | 10 | 0.519 | 1.109 | 2.14× |
| 1,000 | 1 | 0.078 | 0.151 | 1.94× |
| 1,000 | 5 | 0.296 | 0.635 | 2.14× |
| 1,000 | 10 | 0.541 | 1.250 | 2.31× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
