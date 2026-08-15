# CandleHaramiCross benchmark (`CDLHARAMICROSS` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.007 | 152.52M | 0.003 | 308.53M | 0.035 | 5.33× | 10.79× |
| 10,000 | 0.065 | 154.09M | 0.060 | 166.94M | 0.140 | 2.15× | 2.33× |
| 100,000 | 0.869 | 115.13M | 0.820 | 121.94M | 1.048 | 1.21× | 1.28× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.079 | 0.149 | 1.89× |
| 1 | 5 | 0.261 | 0.441 | 1.69× |
| 1 | 10 | 0.385 | 0.890 | 2.31× |
| 10 | 1 | 0.047 | 0.088 | 1.88× |
| 10 | 5 | 0.178 | 0.410 | 2.30× |
| 10 | 10 | 0.405 | 0.873 | 2.16× |
| 100 | 1 | 0.043 | 0.100 | 2.30× |
| 100 | 5 | 0.170 | 0.418 | 2.46× |
| 100 | 10 | 0.385 | 0.969 | 2.51× |
| 1,000 | 1 | 0.069 | 0.117 | 1.69× |
| 1,000 | 5 | 0.236 | 0.517 | 2.19× |
| 1,000 | 10 | 0.452 | 1.072 | 2.37× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
