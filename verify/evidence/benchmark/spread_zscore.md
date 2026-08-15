# SpreadZScore benchmark (`rolling hedged-spread z-score` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.092 | 10.85M | 0.091 | 10.93M | 0.450 | 4.88× | 4.92× |
| 10,000 | 0.926 | 10.80M | 0.898 | 11.14M | 2.987 | 3.23× | 3.33× |
| 100,000 | 9.416 | 10.62M | 9.004 | 11.11M | 35.102 | 3.73× | 3.90× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.066 | 0.208 | 3.13× |
| 1 | 5 | 0.234 | 0.767 | 3.28× |
| 1 | 10 | 0.379 | 1.637 | 4.33× |
| 10 | 1 | 0.043 | 0.156 | 3.68× |
| 10 | 5 | 0.197 | 0.756 | 3.84× |
| 10 | 10 | 0.409 | 1.594 | 3.90× |
| 100 | 1 | 0.056 | 0.264 | 4.72× |
| 100 | 5 | 0.199 | 1.472 | 7.40× |
| 100 | 10 | 0.435 | 3.009 | 6.92× |
| 1,000 | 1 | 0.140 | 0.527 | 3.77× |
| 1,000 | 5 | 0.257 | 1.960 | 7.62× |
| 1,000 | 10 | 0.478 | 4.095 | 8.57× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
