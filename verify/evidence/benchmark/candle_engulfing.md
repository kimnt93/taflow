# CandleEngulfing benchmark (`CDLENGULFING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 192.86M | 0.002 | 540.64M | 0.032 | 6.26× | 17.55× |
| 10,000 | 0.015 | 652.17M | 0.010 | 982.39M | 0.086 | 5.59× | 8.43× |
| 100,000 | 0.111 | 903.69M | 0.095 | 1.05G | 0.577 | 5.21× | 6.04× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.162 | 1.80× |
| 1 | 5 | 0.215 | 0.438 | 2.04× |
| 1 | 10 | 0.397 | 0.875 | 2.21× |
| 10 | 1 | 0.039 | 0.086 | 2.20× |
| 10 | 5 | 0.185 | 0.464 | 2.51× |
| 10 | 10 | 0.417 | 0.896 | 2.15× |
| 100 | 1 | 0.039 | 0.084 | 2.16× |
| 100 | 5 | 0.183 | 0.433 | 2.36× |
| 100 | 10 | 0.381 | 0.963 | 2.52× |
| 1,000 | 1 | 0.056 | 0.090 | 1.60× |
| 1,000 | 5 | 0.189 | 0.484 | 2.55× |
| 1,000 | 10 | 0.403 | 1.044 | 2.59× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
