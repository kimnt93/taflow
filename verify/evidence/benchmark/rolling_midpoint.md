# RollingMidpoint benchmark (`MIDPOINT` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 113.57M | 0.008 | 126.04M | 0.039 | 4.39× | 4.87× |
| 10,000 | 0.087 | 114.77M | 0.085 | 118.24M | 0.107 | 1.23× | 1.27× |
| 100,000 | 0.856 | 116.88M | 0.856 | 116.81M | 0.719 | 0.84× | 0.84× |
| 1,000,000 | 10.394 | 96.21M | 9.801 | 102.03M | 7.540 | 0.73× | 0.77× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.061 | 0.141 | 2.29× |
| 1 | 5 | 0.246 | 0.497 | 2.02× |
| 1 | 10 | 0.487 | 1.063 | 2.18× |
| 10 | 1 | 0.060 | 0.106 | 1.75× |
| 10 | 5 | 0.251 | 0.523 | 2.09× |
| 10 | 10 | 0.492 | 0.950 | 1.93× |
| 100 | 1 | 0.055 | 0.097 | 1.78× |
| 100 | 5 | 0.295 | 0.512 | 1.74× |
| 100 | 10 | 0.517 | 0.959 | 1.85× |
| 1,000 | 1 | 0.056 | 0.097 | 1.73× |
| 1,000 | 5 | 0.237 | 0.583 | 2.45× |
| 1,000 | 10 | 0.608 | 1.046 | 1.72× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
