# CandleHighWave benchmark (`CDLHIGHWAVE` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.006 | 159.96M | 0.003 | 316.04M | 0.038 | 6.07× | 11.99× |
| 10,000 | 0.090 | 111.54M | 0.086 | 115.79M | 0.173 | 1.93× | 2.00× |
| 100,000 | 1.046 | 95.60M | 1.088 | 91.88M | 1.339 | 1.28× | 1.23× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.075 | 0.101 | 1.34× |
| 1 | 5 | 0.290 | 0.456 | 1.57× |
| 1 | 10 | 0.395 | 0.897 | 2.27× |
| 10 | 1 | 0.040 | 0.083 | 2.07× |
| 10 | 5 | 0.208 | 0.422 | 2.03× |
| 10 | 10 | 0.420 | 0.917 | 2.19× |
| 100 | 1 | 0.044 | 0.089 | 2.03× |
| 100 | 5 | 0.183 | 0.442 | 2.42× |
| 100 | 10 | 0.389 | 0.961 | 2.47× |
| 1,000 | 1 | 0.052 | 0.107 | 2.08× |
| 1,000 | 5 | 0.197 | 0.531 | 2.69× |
| 1,000 | 10 | 0.409 | 1.066 | 2.61× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
