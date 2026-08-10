# CandleEveningDojiStar benchmark (`CDLEVENINGDOJISTAR` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.022 | 45.13M | 0.016 | 62.89M | 0.040 | 1.82× | 2.53× |
| 10,000 | 0.161 | 61.97M | 0.155 | 64.57M | 0.116 | 0.72× | 0.75× |
| 100,000 | 1.768 | 56.57M | 1.536 | 65.12M | 0.848 | 0.48× | 0.55× |
| 1,000,000 | 15.669 | 63.82M | 17.338 | 57.68M | 8.975 | 0.57× | 0.52× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.120 | 0.106 | 0.89× |
| 1 | 5 | 0.269 | 0.477 | 1.77× |
| 1 | 10 | 0.539 | 0.971 | 1.80× |
| 10 | 1 | 0.053 | 0.098 | 1.85× |
| 10 | 5 | 0.263 | 0.455 | 1.73× |
| 10 | 10 | 0.558 | 0.944 | 1.69× |
| 100 | 1 | 0.057 | 0.097 | 1.72× |
| 100 | 5 | 0.249 | 0.478 | 1.92× |
| 100 | 10 | 0.560 | 1.009 | 1.80× |
| 1,000 | 1 | 0.072 | 0.108 | 1.50× |
| 1,000 | 5 | 0.285 | 0.521 | 1.82× |
| 1,000 | 10 | 0.587 | 1.084 | 1.85× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
