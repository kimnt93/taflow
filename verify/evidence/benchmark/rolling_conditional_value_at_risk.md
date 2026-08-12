# RollingConditionalValueAtRisk benchmark (`ConditionalValueAtRisk` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.131 | 7.66M | 0.135 | 7.43M | 0.352 | 2.70× | 2.62× |
| 10,000 | 1.362 | 7.34M | 1.381 | 7.24M | 1.820 | 1.34× | 1.32× |
| 100,000 | 13.311 | 7.51M | 13.383 | 7.47M | 18.925 | 1.42× | 1.41× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.107 | 0.335 | 3.14× |
| 1 | 5 | 0.339 | 1.400 | 4.13× |
| 1 | 10 | 0.548 | 3.351 | 6.12× |
| 10 | 1 | 0.054 | 0.251 | 4.66× |
| 10 | 5 | 0.290 | 1.457 | 5.03× |
| 10 | 10 | 0.569 | 2.917 | 5.12× |
| 100 | 1 | 0.066 | 0.285 | 4.29× |
| 100 | 5 | 0.282 | 1.613 | 5.71× |
| 100 | 10 | 0.546 | 3.079 | 5.64× |
| 1,000 | 1 | 0.210 | 0.455 | 2.17× |
| 1,000 | 5 | 0.398 | 2.438 | 6.13× |
| 1,000 | 10 | 0.680 | 4.808 | 7.07× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
