# CandleHomingPigeon benchmark (`CDLHOMINGPIGEON` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.020 | 50.71M | 0.017 | 58.91M | 0.034 | 1.70× | 1.98× |
| 10,000 | 0.120 | 83.18M | 0.118 | 85.04M | 0.098 | 0.82× | 0.84× |
| 100,000 | 1.104 | 90.57M | 1.106 | 90.39M | 0.771 | 0.70× | 0.70× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.189 | 0.117 | 0.62× |
| 1 | 5 | 0.389 | 0.445 | 1.14× |
| 1 | 10 | 0.526 | 0.873 | 1.66× |
| 10 | 1 | 0.057 | 0.083 | 1.45× |
| 10 | 5 | 0.263 | 0.496 | 1.89× |
| 10 | 10 | 0.545 | 0.918 | 1.69× |
| 100 | 1 | 0.054 | 0.088 | 1.63× |
| 100 | 5 | 0.265 | 0.457 | 1.72× |
| 100 | 10 | 0.584 | 0.977 | 1.67× |
| 1,000 | 1 | 0.070 | 0.097 | 1.39× |
| 1,000 | 5 | 0.308 | 0.520 | 1.69× |
| 1,000 | 10 | 0.578 | 1.071 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
