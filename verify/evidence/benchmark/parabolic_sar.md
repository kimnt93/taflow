# ParabolicSar benchmark (`SAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.014 | 73.73M | 0.014 | 73.64M | 0.039 | 2.86× | 2.85× |
| 10,000 | 0.122 | 82.25M | 0.116 | 85.87M | 0.102 | 0.84× | 0.88× |
| 100,000 | 1.173 | 85.24M | 1.125 | 88.91M | 0.661 | 0.56× | 0.59× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.123 | 0.169 | 1.37× |
| 1 | 5 | 0.380 | 0.538 | 1.42× |
| 1 | 10 | 0.515 | 0.994 | 1.93× |
| 10 | 1 | 0.050 | 0.099 | 1.96× |
| 10 | 5 | 0.226 | 0.464 | 2.05× |
| 10 | 10 | 0.474 | 1.027 | 2.17× |
| 100 | 1 | 0.056 | 0.096 | 1.70× |
| 100 | 5 | 0.240 | 0.481 | 2.00× |
| 100 | 10 | 0.505 | 1.035 | 2.05× |
| 1,000 | 1 | 0.071 | 0.118 | 1.67× |
| 1,000 | 5 | 0.235 | 0.502 | 2.14× |
| 1,000 | 10 | 0.512 | 1.079 | 2.11× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
