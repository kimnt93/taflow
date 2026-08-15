# CandleRiseFallThreeMethods benchmark (`CDLRISEFALL3METHODS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 137.07M | 0.004 | 233.68M | 0.033 | 4.52× | 7.70× |
| 10,000 | 0.088 | 113.59M | 0.082 | 122.65M | 0.116 | 1.32× | 1.42× |
| 100,000 | 1.102 | 90.75M | 1.064 | 93.95M | 0.871 | 0.79× | 0.82× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.077 | 0.137 | 1.79× |
| 1 | 5 | 0.307 | 0.457 | 1.49× |
| 1 | 10 | 0.804 | 1.033 | 1.29× |
| 10 | 1 | 0.043 | 0.097 | 2.23× |
| 10 | 5 | 0.200 | 0.440 | 2.20× |
| 10 | 10 | 0.391 | 0.881 | 2.25× |
| 100 | 1 | 0.045 | 0.085 | 1.90× |
| 100 | 5 | 0.182 | 0.404 | 2.23× |
| 100 | 10 | 0.398 | 0.931 | 2.34× |
| 1,000 | 1 | 0.059 | 0.094 | 1.60× |
| 1,000 | 5 | 0.193 | 0.467 | 2.42× |
| 1,000 | 10 | 0.417 | 0.979 | 2.35× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
