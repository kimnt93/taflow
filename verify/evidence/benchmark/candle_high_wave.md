# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.011 | 93.29M | 0.007 | 141.11M | 0.036 | 3.32× | 5.02× |
| 10,000 | 0.130 | 76.71M | 0.128 | 78.14M | 0.165 | 1.26× | 1.29× |
| 100,000 | 1.410 | 70.92M | 1.313 | 76.15M | 1.291 | 0.92× | 0.98× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.136 | 0.123 | 0.90× |
| 1 | 5 | 0.221 | 0.469 | 2.13× |
| 1 | 10 | 0.382 | 0.914 | 2.39× |
| 10 | 1 | 0.041 | 0.089 | 2.14× |
| 10 | 5 | 0.191 | 0.476 | 2.49× |
| 10 | 10 | 0.394 | 0.918 | 2.33× |
| 100 | 1 | 0.048 | 0.091 | 1.92× |
| 100 | 5 | 0.193 | 0.446 | 2.31× |
| 100 | 10 | 0.428 | 0.947 | 2.21× |
| 1,000 | 1 | 0.058 | 0.112 | 1.93× |
| 1,000 | 5 | 0.200 | 0.501 | 2.50× |
| 1,000 | 10 | 0.410 | 1.164 | 2.84× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
