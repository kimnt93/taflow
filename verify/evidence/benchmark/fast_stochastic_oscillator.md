# FastStochasticOscillator benchmark (`STOCHF` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.015 | 67.42M | 0.012 | 82.22M | 0.046 | 3.11× | 3.80× |
| 10,000 | 0.116 | 86.31M | 0.112 | 88.94M | 0.142 | 1.23× | 1.26× |
| 100,000 | 1.876 | 53.31M | 1.081 | 92.54M | 1.032 | 0.55× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.081 | 0.132 | 1.64× |
| 1 | 5 | 0.243 | 0.526 | 2.16× |
| 1 | 10 | 0.423 | 1.127 | 2.66× |
| 10 | 1 | 0.063 | 0.106 | 1.69× |
| 10 | 5 | 0.214 | 0.528 | 2.47× |
| 10 | 10 | 0.434 | 1.099 | 2.53× |
| 100 | 1 | 0.046 | 0.114 | 2.46× |
| 100 | 5 | 0.233 | 0.569 | 2.44× |
| 100 | 10 | 0.480 | 1.068 | 2.22× |
| 1,000 | 1 | 0.059 | 0.122 | 2.08× |
| 1,000 | 5 | 0.263 | 0.568 | 2.16× |
| 1,000 | 10 | 0.434 | 1.160 | 2.67× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
