# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.016 | 62.59M | 0.014 | 72.45M | 0.036 | 2.27× | 2.62× |
| 10,000 | 0.155 | 64.63M | 0.138 | 72.34M | 0.171 | 1.11× | 1.24× |
| 100,000 | 1.479 | 67.60M | 1.391 | 71.90M | 1.437 | 0.97× | 1.03× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.083 | 0.110 | 1.33× |
| 1 | 5 | 0.295 | 0.508 | 1.72× |
| 1 | 10 | 0.609 | 0.916 | 1.50× |
| 10 | 1 | 0.056 | 0.084 | 1.49× |
| 10 | 5 | 0.247 | 0.429 | 1.74× |
| 10 | 10 | 0.590 | 0.908 | 1.54× |
| 100 | 1 | 0.059 | 0.091 | 1.55× |
| 100 | 5 | 0.275 | 0.433 | 1.57× |
| 100 | 10 | 0.544 | 1.034 | 1.90× |
| 1,000 | 1 | 0.081 | 0.111 | 1.36× |
| 1,000 | 5 | 0.256 | 0.534 | 2.09× |
| 1,000 | 10 | 0.595 | 1.117 | 1.88× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
