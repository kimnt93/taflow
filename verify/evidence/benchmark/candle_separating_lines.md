# CandleSeparatingLines benchmark (`CDLSEPARATINGLINES` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 150.23M | 0.003 | 290.12M | 0.034 | 5.06× | 9.77× |
| 10,000 | 0.056 | 179.40M | 0.053 | 187.77M | 0.120 | 2.15× | 2.25× |
| 100,000 | 0.594 | 168.26M | 0.614 | 162.94M | 0.966 | 1.63× | 1.57× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.064 | 0.137 | 2.13× |
| 1 | 5 | 0.248 | 0.449 | 1.81× |
| 1 | 10 | 0.394 | 0.889 | 2.26× |
| 10 | 1 | 0.046 | 0.101 | 2.20× |
| 10 | 5 | 0.180 | 0.437 | 2.43× |
| 10 | 10 | 0.380 | 0.882 | 2.32× |
| 100 | 1 | 0.045 | 0.093 | 2.06× |
| 100 | 5 | 0.177 | 0.420 | 2.37× |
| 100 | 10 | 0.374 | 0.937 | 2.50× |
| 1,000 | 1 | 0.053 | 0.104 | 1.95× |
| 1,000 | 5 | 0.190 | 0.452 | 2.39× |
| 1,000 | 10 | 0.398 | 0.967 | 2.43× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
