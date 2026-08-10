# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.026 | 39.05M | 0.022 | 44.84M | 0.047 | 1.83× | 2.10× |
| 10,000 | 0.185 | 54.06M | 0.218 | 45.90M | 0.132 | 0.71× | 0.61× |
| 100,000 | 1.892 | 52.85M | 1.702 | 58.76M | 0.748 | 0.40× | 0.44× |
| 1,000,000 | 16.573 | 60.34M | 16.830 | 59.42M | 7.226 | 0.44× | 0.43× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.113 | 0.136 | 1.20× |
| 1 | 5 | 0.339 | 0.578 | 1.70× |
| 1 | 10 | 0.655 | 1.108 | 1.69× |
| 10 | 1 | 0.061 | 0.093 | 1.54× |
| 10 | 5 | 0.354 | 0.570 | 1.61× |
| 10 | 10 | 0.639 | 1.219 | 1.91× |
| 100 | 1 | 0.064 | 0.098 | 1.52× |
| 100 | 5 | 0.378 | 0.553 | 1.46× |
| 100 | 10 | 0.722 | 1.227 | 1.70× |
| 1,000 | 1 | 0.103 | 0.124 | 1.21× |
| 1,000 | 5 | 0.417 | 0.673 | 1.61× |
| 1,000 | 10 | 0.700 | 1.238 | 1.77× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
