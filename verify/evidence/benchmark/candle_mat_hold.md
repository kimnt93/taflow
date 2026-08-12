# CandleMatHold benchmark (`CDLMATHOLD` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 39.14M | 0.022 | 45.65M | 0.043 | 1.67× | 1.95× |
| 10,000 | 0.191 | 52.41M | 0.184 | 54.42M | 0.129 | 0.67× | 0.70× |
| 100,000 | 1.786 | 56.00M | 1.751 | 57.11M | 0.881 | 0.49× | 0.50× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.097 | 0.107 | 1.11× |
| 1 | 5 | 0.393 | 0.523 | 1.33× |
| 1 | 10 | 0.515 | 0.951 | 1.85× |
| 10 | 1 | 0.054 | 0.101 | 1.85× |
| 10 | 5 | 0.238 | 0.461 | 1.93× |
| 10 | 10 | 0.578 | 0.970 | 1.68× |
| 100 | 1 | 0.056 | 0.096 | 1.71× |
| 100 | 5 | 0.260 | 0.461 | 1.77× |
| 100 | 10 | 0.529 | 1.031 | 1.95× |
| 1,000 | 1 | 0.082 | 0.106 | 1.28× |
| 1,000 | 5 | 0.289 | 0.496 | 1.72× |
| 1,000 | 10 | 0.543 | 1.038 | 1.91× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
