# CandleUniqueThreeRiver benchmark (`CDLUNIQUE3RIVER` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.005 | 183.07M | 0.002 | 400.27M | 0.030 | 5.52× | 12.06× |
| 10,000 | 0.045 | 223.07M | 0.048 | 208.73M | 0.093 | 2.08× | 1.94× |
| 100,000 | 0.647 | 154.65M | 0.627 | 159.61M | 0.550 | 0.85× | 0.88× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.090 | 0.145 | 1.61× |
| 1 | 5 | 0.325 | 0.456 | 1.40× |
| 1 | 10 | 0.396 | 0.889 | 2.25× |
| 10 | 1 | 0.040 | 0.083 | 2.06× |
| 10 | 5 | 0.173 | 0.406 | 2.34× |
| 10 | 10 | 0.374 | 0.900 | 2.41× |
| 100 | 1 | 0.045 | 0.089 | 1.95× |
| 100 | 5 | 0.179 | 0.413 | 2.30× |
| 100 | 10 | 0.399 | 0.886 | 2.22× |
| 1,000 | 1 | 0.049 | 0.095 | 1.95× |
| 1,000 | 5 | 0.195 | 0.442 | 2.26× |
| 1,000 | 10 | 0.420 | 0.925 | 2.20× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
