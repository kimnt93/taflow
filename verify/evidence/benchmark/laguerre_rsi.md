# LaguerreRelativeStrengthIndex benchmark (`LaguerreRSI` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.010 | 98.40M | 0.008 | 122.16M | 0.171 | 16.87× | 20.94× |
| 10,000 | 0.091 | 109.61M | 0.073 | 136.07M | 0.538 | 5.89× | 7.32× |
| 100,000 | 0.923 | 108.38M | 0.736 | 135.90M | 4.186 | 4.54× | 5.69× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.100 | 0.268 | 2.69× |
| 1 | 5 | 0.318 | 1.366 | 4.30× |
| 1 | 10 | 0.383 | 2.320 | 6.05× |
| 10 | 1 | 0.043 | 0.219 | 5.07× |
| 10 | 5 | 0.197 | 1.340 | 6.81× |
| 10 | 10 | 0.402 | 2.390 | 5.94× |
| 100 | 1 | 0.043 | 0.219 | 5.14× |
| 100 | 5 | 0.196 | 1.335 | 6.82× |
| 100 | 10 | 0.402 | 2.377 | 5.91× |
| 1,000 | 1 | 0.050 | 0.257 | 5.13× |
| 1,000 | 5 | 0.189 | 1.561 | 8.27× |
| 1,000 | 10 | 0.429 | 2.743 | 6.39× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
