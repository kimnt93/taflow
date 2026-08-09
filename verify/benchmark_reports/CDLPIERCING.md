# CandlePiercing benchmark (`CDLPIERCING` oracle)

Correctness: **MATCH**.

taflow class.extend over contiguous NumPy arrays; this exercises the compiled Rust bulk/SIMD-capable path. SIMD availability and target features depend on the installed wheel/build flags.

## Whole-vector performance

| Bars | TAFlow API ms | API bars/s | TAFlow kernel ms | Kernel bars/s | Reference ms | API speedup | Kernel speedup |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,000 | 0.009 | 107.94M | 0.008 | 129.26M | 0.033 | 3.52× | 4.22× |
| 10,000 | 0.079 | 126.35M | 0.075 | 133.10M | 0.121 | 1.52× | 1.61× |
| 100,000 | 0.879 | 113.74M | 0.855 | 116.94M | 1.001 | 1.14× | 1.17× |
| 1,000,000 | 9.190 | 108.82M | 9.287 | 107.68M | 9.480 | 1.03× | 1.02× |

## Fresh-state warm-up

| Bars | Threads | TAFlow ms | Reference ms | Speedup |
|---:|---:|---:|---:|---:|
| 1 | 1 | 0.118 | 0.130 | 1.10× |
| 1 | 5 | 0.311 | 0.492 | 1.58× |
| 1 | 10 | 0.516 | 0.953 | 1.85× |
| 10 | 1 | 0.054 | 0.089 | 1.65× |
| 10 | 5 | 0.251 | 0.456 | 1.82× |
| 10 | 10 | 0.527 | 0.958 | 1.82× |
| 100 | 1 | 0.053 | 0.093 | 1.75× |
| 100 | 5 | 0.264 | 0.462 | 1.75× |
| 100 | 10 | 0.616 | 0.983 | 1.59× |
| 1,000 | 1 | 0.064 | 0.107 | 1.67× |
| 1,000 | 5 | 0.272 | 0.527 | 1.94× |
| 1,000 | 10 | 0.606 | 1.144 | 1.89× |

---
Times include Python conversion/binding overhead. Raw samples are retained in JSON.
