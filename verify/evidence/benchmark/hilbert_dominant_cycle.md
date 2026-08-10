# HilbertDominantCycle benchmark (`HilbertDominantCycle` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.061 | 16.29M | 0.060 | 16.79M | 0.176 | 2.87× | 2.96× |
| 10,000 | 0.585 | 17.09M | 0.563 | 17.76M | 0.936 | 1.60× | 1.66× |
| 100,000 | 6.000 | 16.67M | 5.759 | 17.37M | 8.630 | 1.44× | 1.50× |
| 1,000,000 | 61.089 | 16.37M | 57.068 | 17.52M | 86.901 | 1.42× | 1.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.180 | 0.212 | 1.18× |
| 1 | 5 | 0.277 | 0.840 | 3.03× |
| 1 | 10 | 0.486 | 1.775 | 3.65× |
| 10 | 1 | 0.047 | 0.167 | 3.53× |
| 10 | 5 | 0.221 | 0.780 | 3.54× |
| 10 | 10 | 0.498 | 2.133 | 4.28× |
| 100 | 1 | 0.060 | 0.168 | 2.79× |
| 100 | 5 | 0.238 | 0.870 | 3.65× |
| 100 | 10 | 0.472 | 1.708 | 3.62× |
| 1,000 | 1 | 0.113 | 0.264 | 2.34× |
| 1,000 | 5 | 0.265 | 1.504 | 5.69× |
| 1,000 | 10 | 0.538 | 2.559 | 4.76× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
