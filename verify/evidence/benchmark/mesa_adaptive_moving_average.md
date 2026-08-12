# MesaAdaptiveMovingAverage benchmark (`MAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.057 | 17.59M | 0.054 | 18.45M | 0.092 | 1.61× | 1.69× |
| 10,000 | 0.566 | 17.68M | 0.553 | 18.09M | 0.587 | 1.04× | 1.06× |
| 100,000 | 5.728 | 17.46M | 5.919 | 16.89M | 5.477 | 0.96× | 0.93× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.065 | 0.170 | 2.63× |
| 1 | 5 | 0.317 | 0.541 | 1.71× |
| 1 | 10 | 0.477 | 0.988 | 2.07× |
| 10 | 1 | 0.047 | 0.099 | 2.10× |
| 10 | 5 | 0.217 | 0.487 | 2.24× |
| 10 | 10 | 0.519 | 0.994 | 1.91× |
| 100 | 1 | 0.052 | 0.111 | 2.14× |
| 100 | 5 | 0.245 | 0.525 | 2.14× |
| 100 | 10 | 0.517 | 1.171 | 2.26× |
| 1,000 | 1 | 0.108 | 0.161 | 1.48× |
| 1,000 | 5 | 0.258 | 0.768 | 2.98× |
| 1,000 | 10 | 0.528 | 1.631 | 3.09× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
