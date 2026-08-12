# CandleKicking benchmark (`CDLKICKING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.41M | 0.019 | 52.63M | 0.043 | 1.93× | 2.24× |
| 10,000 | 0.179 | 55.85M | 0.174 | 57.58M | 0.192 | 1.07× | 1.11× |
| 100,000 | 1.838 | 54.40M | 1.793 | 55.78M | 1.653 | 0.90× | 0.92× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.111 | 0.122 | 1.10× |
| 1 | 5 | 0.269 | 0.482 | 1.79× |
| 1 | 10 | 0.572 | 0.963 | 1.69× |
| 10 | 1 | 0.063 | 0.101 | 1.62× |
| 10 | 5 | 0.287 | 0.453 | 1.58× |
| 10 | 10 | 0.542 | 0.950 | 1.75× |
| 100 | 1 | 0.066 | 0.093 | 1.42× |
| 100 | 5 | 0.315 | 0.499 | 1.58× |
| 100 | 10 | 0.598 | 0.953 | 1.59× |
| 1,000 | 1 | 0.073 | 0.111 | 1.54× |
| 1,000 | 5 | 0.294 | 0.549 | 1.87× |
| 1,000 | 10 | 0.643 | 1.097 | 1.71× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
