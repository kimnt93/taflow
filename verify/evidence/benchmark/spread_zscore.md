# SpreadZScore benchmark (`rolling hedged-spread z-score` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.095 | 10.48M | 0.088 | 11.32M | 0.416 | 4.36× | 4.71× |
| 10,000 | 0.875 | 11.43M | 0.888 | 11.26M | 2.817 | 3.22× | 3.17× |
| 100,000 | 8.782 | 11.39M | 8.662 | 11.54M | 32.557 | 3.71× | 3.76× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.055 | 0.188 | 3.40× |
| 1 | 5 | 0.284 | 0.832 | 2.93× |
| 1 | 10 | 0.387 | 1.544 | 3.98× |
| 10 | 1 | 0.042 | 0.162 | 3.86× |
| 10 | 5 | 0.189 | 0.749 | 3.96× |
| 10 | 10 | 0.412 | 1.621 | 3.94× |
| 100 | 1 | 0.056 | 0.257 | 4.63× |
| 100 | 5 | 0.194 | 1.519 | 7.83× |
| 100 | 10 | 0.444 | 2.949 | 6.64× |
| 1,000 | 1 | 0.145 | 0.524 | 3.62× |
| 1,000 | 5 | 0.262 | 1.790 | 6.83× |
| 1,000 | 10 | 0.471 | 4.009 | 8.51× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
