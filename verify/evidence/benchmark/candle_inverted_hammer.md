# CandleInvertedHammer benchmark (`CDLINVERTEDHAMMER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 49.31M | 0.016 | 62.29M | 0.045 | 2.23× | 2.82× |
| 10,000 | 0.170 | 58.89M | 0.165 | 60.78M | 0.193 | 1.14× | 1.18× |
| 100,000 | 1.693 | 59.07M | 1.661 | 60.19M | 1.595 | 0.94× | 0.96× |
| 1,000,000 | 17.406 | 57.45M | 16.844 | 59.37M | 15.270 | 0.88× | 0.91× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.105 | 0.116 | 1.10× |
| 1 | 5 | 0.337 | 0.487 | 1.44× |
| 1 | 10 | 0.568 | 0.907 | 1.60× |
| 10 | 1 | 0.053 | 0.092 | 1.74× |
| 10 | 5 | 0.248 | 0.412 | 1.66× |
| 10 | 10 | 0.594 | 0.969 | 1.63× |
| 100 | 1 | 0.059 | 0.092 | 1.57× |
| 100 | 5 | 0.264 | 0.440 | 1.67× |
| 100 | 10 | 0.568 | 1.075 | 1.89× |
| 1,000 | 1 | 0.074 | 0.113 | 1.52× |
| 1,000 | 5 | 0.287 | 0.510 | 1.77× |
| 1,000 | 10 | 0.624 | 1.182 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
