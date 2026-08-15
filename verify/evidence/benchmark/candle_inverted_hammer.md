# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.013 | 77.86M | 0.010 | 104.08M | 0.042 | 3.23× | 4.32× |
| 10,000 | 0.148 | 67.36M | 0.138 | 72.53M | 0.196 | 1.32× | 1.42× |
| 100,000 | 1.517 | 65.94M | 1.453 | 68.81M | 1.399 | 0.92× | 0.96× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.076 | 0.158 | 2.06× |
| 1 | 5 | 0.251 | 0.485 | 1.93× |
| 1 | 10 | 0.403 | 0.902 | 2.24× |
| 10 | 1 | 0.047 | 0.092 | 1.95× |
| 10 | 5 | 0.192 | 0.445 | 2.32× |
| 10 | 10 | 0.418 | 0.926 | 2.22× |
| 100 | 1 | 0.047 | 0.095 | 2.02× |
| 100 | 5 | 0.206 | 0.459 | 2.23× |
| 100 | 10 | 0.403 | 1.001 | 2.48× |
| 1,000 | 1 | 0.058 | 0.105 | 1.82× |
| 1,000 | 5 | 0.188 | 0.542 | 2.88× |
| 1,000 | 10 | 0.424 | 1.190 | 2.81× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
