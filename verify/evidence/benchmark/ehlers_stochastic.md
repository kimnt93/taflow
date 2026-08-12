# EhlersStochastic benchmark (`EhlersStochastic` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.039 | 25.61M | 0.039 | 25.56M | 0.185 | 4.75× | 4.74× |
| 10,000 | 0.378 | 26.47M | 0.370 | 27.02M | 0.782 | 2.07× | 2.11× |
| 100,000 | 3.530 | 28.33M | 3.739 | 26.75M | 6.905 | 1.96× | 1.85× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.166 | 0.228 | 1.38× |
| 1 | 5 | 0.326 | 0.959 | 2.94× |
| 1 | 10 | 0.509 | 2.203 | 4.33× |
| 10 | 1 | 0.054 | 0.198 | 3.66× |
| 10 | 5 | 0.257 | 0.968 | 3.77× |
| 10 | 10 | 0.482 | 2.326 | 4.82× |
| 100 | 1 | 0.055 | 0.204 | 3.71× |
| 100 | 5 | 0.266 | 1.026 | 3.85× |
| 100 | 10 | 0.517 | 2.212 | 4.28× |
| 1,000 | 1 | 0.089 | 0.265 | 2.97× |
| 1,000 | 5 | 0.256 | 1.354 | 5.29× |
| 1,000 | 10 | 0.557 | 3.112 | 5.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
