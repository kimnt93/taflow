# CandleThreeInside benchmark (`CDL3INSIDE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.021 | 47.32M | 0.015 | 66.56M | 0.046 | 2.16× | 3.04× |
| 10,000 | 0.125 | 79.91M | 0.115 | 86.87M | 0.200 | 1.60× | 1.74× |
| 100,000 | 1.577 | 63.41M | 1.343 | 74.46M | 1.316 | 0.83× | 0.98× |
| 1,000,000 | 12.637 | 79.13M | 12.192 | 82.02M | 14.172 | 1.12× | 1.16× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.108 | 0.170 | 1.57× |
| 1 | 5 | 0.646 | 0.646 | 1.00× |
| 1 | 10 | 1.271 | 2.086 | 1.64× |
| 10 | 1 | 0.090 | 0.112 | 1.25× |
| 10 | 5 | 0.347 | 0.538 | 1.55× |
| 10 | 10 | 2.241 | 1.093 | 0.49× |
| 100 | 1 | 0.087 | 0.093 | 1.07× |
| 100 | 5 | 0.292 | 0.469 | 1.61× |
| 100 | 10 | 0.595 | 0.991 | 1.66× |
| 1,000 | 1 | 0.080 | 0.104 | 1.30× |
| 1,000 | 5 | 0.290 | 0.521 | 1.79× |
| 1,000 | 10 | 0.599 | 1.116 | 1.86× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
