# SuperSmoother benchmark (`SuperSmoother` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 161.34M | 0.005 | 193.64M | 0.146 | 23.56× | 28.27× |
| 10,000 | 0.046 | 218.80M | 0.044 | 226.34M | 0.450 | 9.84× | 10.18× |
| 100,000 | 0.452 | 221.10M | 0.417 | 239.90M | 3.360 | 7.43× | 8.06× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.056 | 0.212 | 3.79× |
| 1 | 5 | 0.326 | 0.967 | 2.97× |
| 1 | 10 | 0.403 | 2.081 | 5.16× |
| 10 | 1 | 0.046 | 0.214 | 4.69× |
| 10 | 5 | 0.238 | 0.986 | 4.15× |
| 10 | 10 | 0.411 | 2.146 | 5.22× |
| 100 | 1 | 0.052 | 0.207 | 3.96× |
| 100 | 5 | 0.203 | 0.960 | 4.73× |
| 100 | 10 | 0.405 | 2.130 | 5.26× |
| 1,000 | 1 | 0.053 | 0.290 | 5.48× |
| 1,000 | 5 | 0.207 | 1.098 | 5.30× |
| 1,000 | 10 | 0.425 | 2.546 | 5.99× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
