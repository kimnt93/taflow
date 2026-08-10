# KaufmanAdaptiveMovingAverage benchmark (`KAMA` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 138.37M | 0.007 | 148.73M | 0.036 | 5.04× | 5.41× |
| 10,000 | 0.042 | 240.74M | 0.038 | 261.43M | 0.067 | 1.61× | 1.75× |
| 100,000 | 0.525 | 190.34M | 0.352 | 283.86M | 0.378 | 0.72× | 1.07× |
| 1,000,000 | 4.089 | 244.57M | 4.003 | 249.83M | 4.043 | 0.99× | 1.01× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.099 | 0.149 | 1.51× |
| 1 | 5 | 0.293 | 0.601 | 2.05× |
| 1 | 10 | 0.511 | 1.081 | 2.12× |
| 10 | 1 | 0.052 | 0.099 | 1.91× |
| 10 | 5 | 0.261 | 0.614 | 2.35× |
| 10 | 10 | 0.647 | 1.103 | 1.71× |
| 100 | 1 | 0.052 | 0.093 | 1.79× |
| 100 | 5 | 0.348 | 0.588 | 1.69× |
| 100 | 10 | 2.819 | 2.776 | 0.98× |
| 1,000 | 1 | 0.063 | 0.131 | 2.06× |
| 1,000 | 5 | 0.379 | 1.008 | 2.66× |
| 1,000 | 10 | 1.048 | 1.117 | 1.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
