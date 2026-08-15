# HighestSince benchmark (`highest since condition` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 164.78M | 0.005 | 216.95M | 0.302 | 49.76× | 65.51× |
| 10,000 | 0.038 | 265.11M | 0.034 | 294.29M | 2.804 | 74.34× | 82.53× |
| 100,000 | 0.357 | 279.87M | 0.326 | 307.05M | 27.553 | 77.11× | 84.60× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.088 | 1.37× |
| 1 | 5 | 0.211 | 0.321 | 1.52× |
| 1 | 10 | 0.375 | 0.706 | 1.88× |
| 10 | 1 | 0.044 | 0.073 | 1.67× |
| 10 | 5 | 0.171 | 0.329 | 1.92× |
| 10 | 10 | 0.385 | 0.687 | 1.78× |
| 100 | 1 | 0.040 | 0.093 | 2.30× |
| 100 | 5 | 0.188 | 0.508 | 2.70× |
| 100 | 10 | 0.413 | 0.934 | 2.26× |
| 1,000 | 1 | 0.047 | 0.348 | 7.32× |
| 1,000 | 5 | 0.186 | 1.769 | 9.53× |
| 1,000 | 10 | 0.408 | 3.440 | 8.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
